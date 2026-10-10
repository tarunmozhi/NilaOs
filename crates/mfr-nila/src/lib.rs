use anyhow::{bail, Result};
use mhr_core::{sha256_file, MhfManifest};
use std::path::Path;

pub fn verify_payload(path: &Path, expected_sha256: &str) -> Result<()> {
    if expected_sha256.len() != 64
        || !expected_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
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
        "mar" => "Maha Android Runtime",
        "linux" => "Nila Linux Runtime",
        "native" => "Nila Native Runtime",
        _ => "Unsupported Runtime Adapter",
    }
}

pub fn authorize(m: &MhfManifest) -> Result<()> {
    if !m.supported {
        bail!("The application is not supported by this Nila build");
    }
    if m.sha256.len() != 64 || !m.sha256.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("MHF package must contain a valid SHA-256 integrity digest");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mhr_core::InputFormat;
    use std::{fs, time::{SystemTime, UNIX_EPOCH}};

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
        }
    }

    fn temp_payload(contents: &[u8]) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mfr-nila-{}-{nonce}.bin",
            std::process::id()
        ));
        fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn selects_android_runtime() {
        assert_eq!(
            select_runtime(&manifest(
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
                true
            )),
            "Maha Android Runtime"
        );
    }

    #[test]
    fn verifies_matching_payload_digest() {
        let path = temp_payload(b"abc");
        let result = verify_payload(
            &path,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        );
        let _ = fs::remove_file(path);
        assert!(result.is_ok());
    }

    #[test]
    fn accepts_uppercase_hex_digest() {
        let path = temp_payload(b"abc");
        let result = verify_payload(
            &path,
            "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD",
        );
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
    fn authorization_rejects_malformed_digest() {
        assert!(authorize(&manifest("test", true)).is_err());
    }

    #[test]
    fn authorization_rejects_unsupported_applications() {
        assert!(authorize(&manifest(
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            false
        ))
        .is_err());
    }
}
