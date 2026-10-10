use anyhow::{bail, Result};
use mhr_core::{analyze, create_manifest};
use nila_bootstrap::prepare_startup;
use nila_security::load_production_policy;
use std::env;

fn usage() {
    eprintln!(
        "Usage:
  mhr-cli analyze <file>
  mhr-cli manifest <file> <name> <version>
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

            println!(
                "{}",
                serde_json::to_string_pretty(&create_manifest(&analysis, &name, &version))?
            );
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
