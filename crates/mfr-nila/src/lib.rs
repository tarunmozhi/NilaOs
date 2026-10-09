use anyhow::{bail, Result};
use mhr_core::MhfManifest;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
pub fn verify_payload(path: &Path, expected_sha256: &str) -> Result<()> {
    let data = fs::read(path)?;
    let mut h = Sha256::new();
    h.update(data);
    let actual = format!("{:x}", h.finalize());
    if actual != expected_sha256 {
        bail!("MFR integrity verification failed")
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
        bail!("The application is not supported by this Nila build")
    }
    if m.sha256.is_empty() {
        bail!("MHF package does not contain integrity metadata")
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selects_android_runtime() {
        let m = MhfManifest {
            name: "Test Application".into(),
            version: "0.1".into(),
            source_format: mhr_core::InputFormat::Apk,
            runtime: "mar".into(),
            platform: "android".into(),
            abi: "arm64-v8a".into(),
            sha256: "test".into(),
            supported: true,
            notes: Vec::new(),
        };
        assert_eq!(select_runtime(&m), "Maha Android Runtime");
    }
}
