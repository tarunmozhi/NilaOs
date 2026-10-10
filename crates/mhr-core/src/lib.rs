use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{self, Read},
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

pub fn detect_format(path: &Path) -> InputFormat {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
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

/// Hash a file incrementally so large APKs and other payloads do not need to
/// be loaded into RAM in one allocation.
pub fn sha256_file(path: &Path) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];

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

    // Recognition is not execution support. No app runtime is implemented in
    // this baseline, so every detected format remains non-executable.
    let (platform, abi, runtime, note) = match format {
        InputFormat::Apk => (
            "android".into(),
            "arm64-v8a/armeabi-v7a".into(),
            "mar".into(),
            "APK recognized; Maha Android Runtime and Android/vendor compatibility are not implemented."
                .into(),
        ),
        InputFormat::Shell => (
            "linux".into(),
            "arm64".into(),
            "linux".into(),
            "Shell script recognized; an approved Linux runtime is not implemented.".into(),
        ),
        InputFormat::App => (
            "apple".into(),
            "unknown".into(),
            "mfr-adapter".into(),
            "APP format is recognized, but native Apple application execution is not implemented."
                .into(),
        ),
        InputFormat::Exe => (
            "windows".into(),
            "unknown".into(),
            "mfr-adapter".into(),
            "EXE format is recognized, but arbitrary Windows binary translation is not implemented."
                .into(),
        ),
        InputFormat::Unknown => (
            "unknown".into(),
            "unknown".into(),
            "none".into(),
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
        supported: false,
        notes: vec![note],
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            .expect("system clock should be after Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "nila-mhr-{}-{nonce}.{extension}",
            std::process::id()
        ));
        fs::write(&path, contents).expect("temporary test file should be writable");
        path
    }

    #[test]
    fn detects_apk_case_insensitively() {
        assert_eq!(detect_format(Path::new("application.APK")), InputFormat::Apk);
    }

    #[test]
    fn detects_exe() {
        assert_eq!(detect_format(Path::new("application.exe")), InputFormat::Exe);
    }

    #[test]
    fn detects_shell() {
        assert_eq!(detect_format(Path::new("application.sh")), InputFormat::Shell);
    }

    #[test]
    fn hashes_file_contents() {
        let path = temp_file(b"abc", "bin");
        let result = sha256_file(&path);
        let _ = fs::remove_file(&path);
        assert_eq!(
            result.expect("hash should succeed"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn hashes_large_file_without_changing_digest() {
        let contents = vec![b'x'; 2 * 1024 * 1024];
        let path = temp_file(&contents, "bin");
        let result = sha256_file(&path);
        let _ = fs::remove_file(&path);
        let expected = format!("{:x}", Sha256::digest(&contents));
        assert_eq!(result.expect("hash should succeed"), expected);
    }

    #[test]
    fn rejects_missing_file() {
        assert!(sha256_file(Path::new("definitely-not-a-nila-file.bin")).is_err());
    }

    #[test]
    fn recognized_formats_are_not_marked_executable() {
        let path = temp_file(b"not a real APK", "apk");
        let result = analyze(&path);
        let _ = fs::remove_file(&path);
        let analysis = result.expect("analysis should recognize the extension");
        assert_eq!(analysis.format, InputFormat::Apk);
        assert!(!analysis.supported);
    }
}
