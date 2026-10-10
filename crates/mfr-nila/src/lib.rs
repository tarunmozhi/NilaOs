use anyhow::{bail, Context, Result};
use ed25519_dalek::{Signature, VerifyingKey};
use mhr_core::{sha256_file, InputFormat, MhfManifest};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrustedKeyStore {
    /// key ID -> 32-byte Ed25519 public key, encoded as 64 hex characters.
    pub keys: BTreeMap<String, String>,
}

#[derive(Serialize)]
struct SignedManifestPayload<'a> {
    key_id: &'a str,
    name: &'a str,
    version: &'a str,
    source_format: &'a InputFormat,
    runtime: &'a str,
    platform: &'a str,
    abi: &'a str,
    sha256: &'a str,
    supported: bool,
    notes: &'a [String],
}

fn payload_bytes(manifest: &MhfManifest, key_id: &str) -> Result<Vec<u8>> {
    let payload = SignedManifestPayload {
        key_id,
        name: &manifest.name,
        version: &manifest.version,
        source_format: &manifest.source_format,
        runtime: &manifest.runtime,
        platform: &manifest.platform,
        abi: &manifest.abi,
        sha256: &manifest.sha256,
        supported: manifest.supported,
        notes: &manifest.notes,
    };
    serde_json::to_vec(&payload).context("could not serialize signed manifest metadata")
}

fn decode_hex<const N: usize>(value: &str, field: &str) -> Result<[u8; N]> {
    if value.len() != N * 2 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("{field} must be exactly {} hexadecimal characters", N * 2);
    }

    let mut output = [0_u8; N];
    for (index, byte) in output.iter_mut().enumerate() {
        let offset = index * 2;
        *byte = u8::from_str_radix(&value[offset..offset + 2], 16)
            .with_context(|| format!("{field} contains invalid hexadecimal data"))?;
    }
    Ok(output)
}

fn valid_key_id(key_id: &str) -> bool {
    !key_id.is_empty()
        && key_id.len() <= 64
        && key_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

pub fn load_trusted_keys(path: impl AsRef<Path>) -> Result<TrustedKeyStore> {
    let contents = fs::read_to_string(path).context("could not read trusted-key store")?;
    let store: TrustedKeyStore =
        toml::from_str(&contents).context("invalid trusted-key store TOML")?;

    for (key_id, public_key) in &store.keys {
        if !valid_key_id(key_id) {
            bail!("trusted-key ID contains invalid characters");
        }
        let raw = decode_hex::<32>(public_key, "trusted Ed25519 public key")?;
        VerifyingKey::from_bytes(&raw).context("invalid trusted Ed25519 public key")?;
    }

    Ok(store)
}

/// Verify a manifest signature against an explicitly provisioned trusted-key
/// map. An empty key store trusts nobody. Keys are public and may be committed;
/// signing secrets must never be committed or distributed with the OS.
pub fn verify_manifest_signature(
    manifest: &MhfManifest,
    trusted_keys: &BTreeMap<String, String>,
) -> Result<()> {
    let signature_metadata = manifest
        .signature
        .as_ref()
        .context("MHF manifest has no package signature")?;

    if !valid_key_id(&signature_metadata.key_id) {
        bail!("invalid package signing key ID");
    }

    let public_key_hex = trusted_keys
        .get(&signature_metadata.key_id)
        .context("package was signed by an untrusted key")?;
    let public_key_bytes = decode_hex::<32>(public_key_hex, "trusted Ed25519 public key")?;
    let public_key = VerifyingKey::from_bytes(&public_key_bytes)
        .context("invalid trusted Ed25519 public key")?;
    let signature_bytes = decode_hex::<64>(&signature_metadata.signature_hex, "package signature")?;
    let signature = Signature::from_bytes(&signature_bytes);
    let payload = payload_bytes(manifest, &signature_metadata.key_id)?;

    public_key
        .verify_strict(&payload, &signature)
        .context("package signature verification failed")
}

pub fn verify_payload(path: &Path, expected_sha256: &str) -> Result<()> {
    if expected_sha256.len() != 64 || !expected_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        bail!("MFR integrity metadata must be a 64-character SHA-256 hex digest");
    }

    let actual = sha256_file(path)?;
    if !actual.eq_ignore_ascii_case(expected_sha256) {
        bail!("MFR integrity verification failed");
    }

    Ok(())
}

pub fn select_runtime(m: &MhfManifest) -> &'static str {
    match m.runtime.as_str() {
        "mar" => "Maha Android Runtime (not implemented)",
        "linux" => "Nila Linux Runtime (not implemented)",
        "native" => "Nila Native Runtime (not implemented)",
        _ => "Unsupported Runtime Adapter",
    }
}

/// Refuse authorization unless metadata is well-formed and signed by a
/// provisioned trusted key. Even a valid signature cannot enable an absent
/// runtime: this crate intentionally fails closed until an executor exists.
pub fn authorize(m: &MhfManifest, trusted_keys: &BTreeMap<String, String>) -> Result<()> {
    if !m.supported {
        bail!("The application is not supported by this Nila build");
    }
    if m.name.trim().is_empty() || m.version.trim().is_empty() {
        bail!("MHF manifest name and version must not be empty");
    }
    if m.sha256.len() != 64 || !m.sha256.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("MHF package must contain a valid SHA-256 integrity digest");
    }

    verify_manifest_signature(m, trusted_keys)?;

    match m.runtime.as_str() {
        "mar" | "linux" | "native" => {
            bail!(
                "Runtime '{}' is not implemented; refusing execution",
                m.runtime
            )
        }
        _ => bail!("Unsupported runtime '{}'; refusing execution", m.runtime),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    const VALID_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn test_key() -> (SigningKey, BTreeMap<String, String>) {
        let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
        let mut trusted = BTreeMap::new();
        trusted.insert(
            "test-key-1".into(),
            hex(&signing_key.verifying_key().to_bytes()),
        );
        (signing_key, trusted)
    }

    fn manifest(sha256: &str, supported: bool) -> MhfManifest {
        MhfManifest {
            name: "Test Application".into(),
            version: "0.1".into(),
            source_format: InputFormat::Apk,
            runtime: "mar".into(),
            platform: "android".into(),
            abi: "arm64-v8a".into(),
            sha256: sha256.into(),
            supported,
            notes: Vec::new(),
            signature: None,
        }
    }

    fn signed_manifest(signing_key: &SigningKey, sha256: &str, supported: bool) -> MhfManifest {
        let mut manifest = manifest(sha256, supported);
        let payload = payload_bytes(&manifest, "test-key-1").unwrap();
        manifest.signature = Some(mhr_core::PackageSignature {
            key_id: "test-key-1".into(),
            signature_hex: hex(&signing_key.sign(&payload).to_bytes()),
        });
        manifest
    }

    fn temp_payload(contents: &[u8]) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("mfr-nila-{}-{nonce}.bin", std::process::id()));
        fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn selects_android_runtime_but_marks_it_unimplemented() {
        assert_eq!(
            select_runtime(&manifest(VALID_SHA256, true)),
            "Maha Android Runtime (not implemented)"
        );
    }

    #[test]
    fn verifies_matching_payload_digest() {
        let path = temp_payload(b"abc");
        let result = verify_payload(&path, VALID_SHA256);
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }

    #[test]
    fn accepts_uppercase_hex_digest() {
        let path = temp_payload(b"abc");
        let result = verify_payload(&path, &VALID_SHA256.to_ascii_uppercase());
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_mismatched_payload_digest() {
        let path = temp_payload(b"abc");
        let result = verify_payload(&path, &"0".repeat(64));
        let _ = fs::remove_file(path);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_malformed_digest() {
        let path = temp_payload(b"abc");
        let result = verify_payload(&path, "not-a-digest");
        let _ = fs::remove_file(path);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_signature() {
        let (_, trusted) = test_key();
        assert!(verify_manifest_signature(&manifest(VALID_SHA256, true), &trusted).is_err());
    }

    #[test]
    fn verifies_signature_from_trusted_key() {
        let (key, trusted) = test_key();
        let app = signed_manifest(&key, VALID_SHA256, true);
        assert!(verify_manifest_signature(&app, &trusted).is_ok());
    }

    #[test]
    fn rejects_untrusted_signing_key() {
        let (key, _) = test_key();
        let app = signed_manifest(&key, VALID_SHA256, true);
        assert!(verify_manifest_signature(&app, &BTreeMap::new()).is_err());
    }

    #[test]
    fn rejects_tampered_signed_metadata() {
        let (key, trusted) = test_key();
        let mut app = signed_manifest(&key, VALID_SHA256, true);
        app.name = "Tampered Application".into();
        assert!(verify_manifest_signature(&app, &trusted).is_err());
    }

    #[test]
    fn rejects_malformed_signature_hex() {
        let (_, trusted) = test_key();
        let mut app = manifest(VALID_SHA256, true);
        app.signature = Some(mhr_core::PackageSignature {
            key_id: "test-key-1".into(),
            signature_hex: "xyz".into(),
        });
        assert!(verify_manifest_signature(&app, &trusted).is_err());
    }

    #[test]
    fn authorization_rejects_malformed_digest() {
        let (_, trusted) = test_key();
        assert!(authorize(&manifest("test", true), &trusted).is_err());
    }

    #[test]
    fn authorization_rejects_unsupported_applications() {
        let (key, trusted) = test_key();
        assert!(authorize(&signed_manifest(&key, VALID_SHA256, false), &trusted).is_err());
    }

    #[test]
    fn authorization_fails_closed_for_unimplemented_runtime() {
        let (key, trusted) = test_key();
        let error = authorize(&signed_manifest(&key, VALID_SHA256, true), &trusted).unwrap_err();
        assert!(error.to_string().contains("not implemented"));
    }

    #[test]
    fn authorization_rejects_unknown_runtime() {
        let (key, trusted) = test_key();
        let mut app = signed_manifest(&key, VALID_SHA256, true);
        app.runtime = "made-up-runtime".into();
        // A signature cannot be reused after any signed metadata changes.
        assert!(authorize(&app, &trusted).is_err());
    }

    #[test]
    fn authorization_rejects_empty_metadata() {
        let (key, trusted) = test_key();
        let mut app = signed_manifest(&key, VALID_SHA256, true);
        app.name = "  ".into();
        assert!(authorize(&app, &trusted).is_err());
    }
}
