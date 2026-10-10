use anyhow::{bail, Context, Result};
use mfr_nila::{load_trusted_keys, verify_manifest_signature};
use mhr_core::{analyze, create_manifest, MhfManifest};
use nila_bootstrap::prepare_startup;
use nila_security::load_production_policy;
use std::{env, fs};

fn usage() {
    eprintln!(
        "Usage:
  mhr-cli analyze <file>
  mhr-cli manifest <file> <name> <version>
  mhr-cli verify-manifest <manifest.json> <trusted-keys.toml>
  mhr-cli check-policy [path]
  mhr-cli startup-check [policy-path]"
    );
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);

    match args.next().unwrap_or_default().as_str() {
        "analyze" => {
            let file = args.next().ok_or_else(|| anyhow::anyhow!("missing file"))?;
            println!("{}", serde_json::to_string_pretty(&analyze(&file)?)?);
        }
        "manifest" => {
            let file = args.next().ok_or_else(|| anyhow::anyhow!("missing file"))?;
            let name = args.next().ok_or_else(|| anyhow::anyhow!("missing name"))?;
            let version = args
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing version"))?;
            let analysis = analyze(&file)?;

            if !analysis.supported {
                bail!("input format is recognized but unsupported by this build");
            }

            println!(
                "{}",
                serde_json::to_string_pretty(&create_manifest(&analysis, &name, &version))?
            );
        }
        "verify-manifest" => {
            let manifest_path = args
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing manifest JSON path"))?;
            let keys_path = args
                .next()
                .ok_or_else(|| anyhow::anyhow!("missing trusted-key TOML path"))?;

            let manifest_bytes = fs::read(&manifest_path)
                .with_context(|| format!("could not read manifest: {manifest_path}"))?;
            let manifest: MhfManifest = serde_json::from_slice(&manifest_bytes)
                .context("manifest JSON is invalid or has an incompatible schema")?;
            let trusted_keys = load_trusted_keys(&keys_path)?;
            verify_manifest_signature(&manifest, &trusted_keys.keys)?;

            println!("MHF manifest signature: VALID");
            println!("Signer key ID: {}", manifest.signature.as_ref().unwrap().key_id);
            println!("Important: this validates signed metadata only; it does not verify a payload, install an app, or prove that its runtime is implemented.");
        }
        "check-policy" => {
            let path = args
                .next()
                .unwrap_or_else(|| "config/nila-security.toml".to_owned());
            let policy =
                load_production_policy(&path).map_err(|error| anyhow::anyhow!("{error}"))?;
            policy
                .validate_production()
                .map_err(|error| anyhow::anyhow!("{error}"))?;

            println!("Nila OS security policy: VALID");
            println!("Configuration: {path}");
            println!("SELinux enforcement: enabled");
            println!("Signed-package requirement: enabled");
            println!("Package integrity checks: enabled");
            println!("Root access for apps: denied");
            println!("Privileged-action auditing: enabled");
        }
        "startup-check" => {
            let path = args
                .next()
                .unwrap_or_else(|| "config/nila-security.toml".to_owned());
            let services = prepare_startup(&path)
                .map_err(|error| anyhow::anyhow!("Startup refused: {error}"))?;

            println!("Nila OS startup preparation: VALID");
            println!("Validated service entries: {}", services.len());

            for service in services {
                println!(
                    "- {} [{}]",
                    service.name,
                    if service.privileged {
                        "privileged"
                    } else {
                        "unprivileged"
                    }
                );
            }
        }
        _ => {
            usage();
            bail!("unknown command");
        }
    }

    Ok(())
}
