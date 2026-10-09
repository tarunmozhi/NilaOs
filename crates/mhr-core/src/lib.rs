use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
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
pub fn sha256_file(path: &Path) -> anyhow::Result<String> {
    let data = fs::read(path)?;
    let mut h = Sha256::new();
    h.update(data);
    Ok(format!("{:x}", h.finalize()))
}
pub fn analyze(path: impl AsRef<Path>) -> anyhow::Result<Analysis> {
    let path = path.as_ref().to_path_buf();
    let format = detect_format(&path);
    let sha256 = sha256_file(&path)?;
    let (platform,abi,runtime,supported,note)=match format { InputFormat::Apk=>("android".into(),"arm64-v8a/armeabi-v7a".into(),"mar".into(),true,"APK requires Android compatibility support; proprietary Google or vendor services may be unavailable.".into()), InputFormat::Shell=>("linux".into(),"arm64".into(),"linux".into(),true,"Shell applications require an approved Linux runtime.".into()), InputFormat::App=>("apple".into(),"unknown".into(),"mfr-adapter".into(),false,"APP format is recognized, but native Apple application execution is not implemented.".into()), InputFormat::Exe=>("windows".into(),"unknown".into(),"mfr-adapter".into(),false,"EXE format is recognized, but arbitrary Windows binary translation is not implemented.".into()), InputFormat::Unknown=>("unknown".into(),"unknown".into(),"none".into(),false,"Unsupported application format.".into()) };
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
    #[test]
    fn detects_apk() {
        assert_eq!(
            detect_format(Path::new("application.apk")),
            InputFormat::Apk
        )
    }
    #[test]
    fn detects_exe() {
        assert_eq!(
            detect_format(Path::new("application.exe")),
            InputFormat::Exe
        )
    }
    #[test]
    fn detects_shell() {
        assert_eq!(
            detect_format(Path::new("application.sh")),
            InputFormat::Shell
        )
    }
}
