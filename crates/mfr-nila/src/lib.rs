use anyhow::{bail, Result};
use mhr_core::MhfManifest;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

pub fn verify_payload(path: &Path, expected_sha256: &str) -> Result<()> {
    if !is_sha256_hex(expected_sha256) {
        bail!("MHF integrity metadata must be a 64-character SHA-256 hex digest");
    }

    let data = fs::read(path)?;
    let mut h = Sha256::new();
    h.update(data);
    let actual = format!("{:x}", h.finalize());

    if !actual.eq_ignore_ascii_case(expected_sha256) {
        bail!("MFR integrity verification failed");
    }
    Ok(())
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit())
}

pub fn select_runtime(m: &MhfManifest) -> &'static str {
    match m.runtime.as_str() {
        "mar" if m.platform == "android" => "Maha Android Runtime",
        "linux" if m.platform == "linux" => "Nila Linux Runtime",
        "native" if m.platform == "nila" => "Nila Native Runtime",
        _ => "Unsupported Runtime Adapter",
    }
}

pub fn authorize(m: &MhfManifest) -> Result<()> {
    if !m.supported {
        bail!("The application is not supported by this Nila build");
    }
    if !is_sha256_hex(&m.sha256) {
        bail!("MHF package must contain a valid 64-character SHA-256 digest");
    }
    if select_runtime(m) == "Unsupported Runtime Adapter" {
        bail!("MHF package requests an unsupported or mismatched runtime/platform");
    }
    if m.name.trim().is_empty() || m.version.trim().is_empty() {
        bail!("MHF package name and version must not be empty");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(runtime: &str, platform: &str, sha256: &str) -> MhfManifest {
        MhfManifest {
            name: "Test Application".into(),
            version: "0.1".into(),
            source_format: mhr_core::InputFormat::Apk,
            runtime: runtime.into(),
            platform: platform.into(),
            abi: "arm64-v8a".into(),
            sha256: sha256.into(),
            supported: true,
            notes: Vec::new(),
        }
    }

    #[test]
    fn selects_android_runtime() {
        let m = manifest("mar", "android", &"a".repeat(64));
        assert_eq!(select_runtime(&m), "Maha Android Runtime");
        assert!(authorize(&m).is_ok());
    }

    #[test]
    fn rejects_malformed_digest() {
        let m = manifest("mar", "android", "test");
        assert!(authorize(&m).is_err());
    }

    #[test]
    fn rejects_unknown_runtime() {
        let m = manifest("unknown", "android", &"a".repeat(64));
        assert!(authorize(&m).is_err());
    }

    #[test]
    fn rejects_runtime_platform_mismatch() {
        let m = manifest("mar", "linux", &"a".repeat(64));
        assert!(authorize(&m).is_err());
    }

    #[test]
    fn rejects_unsupported_application() {
        let mut m = manifest("mar", "android", &"a".repeat(64));
        m.supported = false;
        assert!(authorize(&m).is_err());
    }

    #[test]
    fn rejects_empty_name_or_version() {
        let mut m = manifest("mar", "android", &"a".repeat(64));
        m.name = "  ".into();
        assert!(authorize(&m).is_err());
    }

    #[test]
    fn rejects_invalid_expected_digest_before_reading_file() {
        assert!(verify_payload(Path::new("nonexistent-file"), "not-a-digest").is_err());
    }
}
