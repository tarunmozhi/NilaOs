use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InputFormat {
    Exe,
    App,
    Apk,
    Shell,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub input: PathBuf,
    pub format: InputFormat,
    pub platform: String,
    pub abi: String,
    pub dependencies: Vec<String>,
    pub runtime: String,
    pub sha256: String,
    pub supported: bool,
    pub notes: Vec<String>,
}

/// Detached Ed25519 signature metadata. The private signing key must never be
/// stored in the repository or on a production device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PackageSignature {
    pub key_id: String,
    /// Lowercase or uppercase hexadecimal encoding of a 64-byte Ed25519 signature.
    pub signature_hex: String,
}

pub fn detect_format(path: &Path) -> InputFormat {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "exe" => InputFormat::Exe,
        "app" => InputFormat::App,
        "apk" => InputFormat::Apk,
        "sh" => InputFormat::Shell,
        _ => InputFormat::Unknown,
    }
}

/// Hash a file incrementally so large inputs do not need to fit in memory.
pub fn sha256_file(path: &Path) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 16 * 1024];

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

pub fn analyze(path: impl AsRef<Path>) -> anyhow::Result<Analysis> {
    let path = path.as_ref().to_path_buf();
    let format = detect_format(&path);
    let sha256 = sha256_file(&path)?;
    let (platform, abi, runtime, supported, note) = match format {
        InputFormat::Apk => (
            "android".into(),
            "arm64-v8a/armeabi-v7a".into(),
            "mar".into(),
            false,
            "APK format is recognized, but Maha Android Runtime (MAR) is not implemented in this baseline.".into(),
        ),
        InputFormat::Shell => (
            "linux".into(),
            "arm64".into(),
            "linux".into(),
            false,
            "Shell format is recognized, but execution requires a Linux runtime that is not implemented in this baseline.".into(),
        ),
        InputFormat::App => (
            "apple".into(),
            "unknown".into(),
            "mfr-adapter".into(),
            false,
            "APP format is recognized, but native Apple application execution is not implemented.".into(),
        ),
        InputFormat::Exe => (
            "windows".into(),
            "unknown".into(),
            "mfr-adapter".into(),
            false,
            "EXE format is recognized, but arbitrary Windows binary translation is not implemented.".into(),
        ),
        InputFormat::Unknown => (
            "unknown".into(),
            "unknown".into(),
            "none".into(),
            false,
            "Unsupported application format.".into(),
        ),
    };

    Ok(Analysis {
        input: path,
        format,
        platform,
        abi,
        dependencies: Vec::new(),
        runtime,
        sha256,
        supported,
        notes: vec![note],
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MhfManifest {
    pub name: String,
    pub version: String,
    pub source_format: InputFormat,
    pub runtime: String,
    pub platform: String,
    pub abi: String,
    pub sha256: String,
    pub supported: bool,
    pub notes: Vec<String>,
    #[serde(default)]
    pub signature: Option<PackageSignature>,
}

pub fn create_manifest(a: &Analysis, name: &str, version: &str) -> MhfManifest {
    MhfManifest {
        name: name.into(),
        version: version.into(),
        source_format: a.format.clone(),
        runtime: a.runtime.clone(),
        platform: a.platform.clone(),
        abi: a.abi.clone(),
        sha256: a.sha256.clone(),
        supported: a.supported,
        notes: a.notes.clone(),
        signature: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_file(contents: &[u8], extension: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mhr-core-{}-{nonce}.{extension}",
            std::process::id()
        ));
        fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn detects_apk() {
        assert_eq!(
            detect_format(Path::new("application.apk")),
            InputFormat::Apk
        );
    }

    #[test]
    fn detects_exe() {
        assert_eq!(
            detect_format(Path::new("application.exe")),
            InputFormat::Exe
        );
    }

    #[test]
    fn detects_shell() {
        assert_eq!(
            detect_format(Path::new("application.sh")),
            InputFormat::Shell
        );
    }

    #[test]
    fn hashes_file_with_known_sha256() {
        let path = temp_file(b"abc", "bin");
        let result = sha256_file(&path);
        let _ = fs::remove_file(path);
        assert_eq!(
            result.unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn analysis_does_not_claim_unimplemented_runtimes_are_supported() {
        for extension in ["apk", "sh"] {
            let path = temp_file(b"placeholder", extension);
            let result = analyze(&path);
            let _ = fs::remove_file(path);
            assert!(
                !result.unwrap().supported,
                "{extension} runtime is not implemented"
            );
        }
    }

    #[test]
    fn new_manifests_are_unsigned_until_explicitly_signed() {
        let path = temp_file(b"placeholder", "apk");
        let analysis = analyze(&path).unwrap();
        let manifest = create_manifest(&analysis, "Example", "0.1");
        let _ = fs::remove_file(path);
        assert!(manifest.signature.is_none());
    }
}
