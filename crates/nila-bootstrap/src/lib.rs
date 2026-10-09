use nila_security::{load_production_policy, SecurityPolicy};
use nila_services::{validated_services, Service};
use std::path::Path;

/// Validates the production policy before returning the service registry.
/// This does not launch services or enforce kernel-level security.
pub fn prepare_startup(policy_path: impl AsRef<Path>) -> Result<Vec<Service>, String> {
    let policy: SecurityPolicy = load_production_policy(policy_path)?;
    validated_services(&policy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    const VALID_POLICY: &str = r#"
selinux_enforcing = true
require_signed_packages = true
require_integrity = true
deny_root_to_apps = true
audit_privileged_actions = true
"#;

    fn temp_policy_path() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("nila-policy-{}-{nonce}.toml", std::process::id()))
    }

    #[test]
    fn startup_accepts_valid_policy() {
        let path = temp_policy_path();
        fs::write(&path, VALID_POLICY).unwrap();

        let result = prepare_startup(&path);
        let _ = fs::remove_file(&path);

        let services = result.expect("valid policy should be accepted");
        assert!(services.iter().any(|s| s.name == "nilad-security"));
    }

    #[test]
    fn startup_rejects_missing_policy() {
        let path = temp_policy_path();
        assert!(prepare_startup(path).is_err());
    }

    #[test]
    fn startup_rejects_weakened_policy() {
        let path = temp_policy_path();
        let weakened =
            VALID_POLICY.replace("selinux_enforcing = true", "selinux_enforcing = false");
        fs::write(&path, weakened).unwrap();

        let result = prepare_startup(&path);
        let _ = fs::remove_file(&path);

        assert!(result.is_err());
    }
}
