use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SecurityPolicy {
    pub selinux_enforcing: bool,
    pub require_signed_packages: bool,
    pub require_integrity: bool,
    pub deny_root_to_apps: bool,
    pub audit_privileged_actions: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            selinux_enforcing: true,
            require_signed_packages: true,
            require_integrity: true,
            deny_root_to_apps: true,
            audit_privileged_actions: true,
        }
    }
}

impl SecurityPolicy {
    pub fn validate_production(&self) -> Result<(), &'static str> {
        if !self.selinux_enforcing {
            return Err("SELinux enforcement must be enabled");
        }
        if !self.require_signed_packages {
            return Err("Signed packages are required");
        }
        if !self.require_integrity {
            return Err("Package integrity verification is required");
        }
        if !self.deny_root_to_apps {
            return Err("Root access must not be granted to apps");
        }
        if !self.audit_privileged_actions {
            return Err("Privileged-action auditing must be enabled");
        }
        Ok(())
    }
}

pub fn production_policy() -> SecurityPolicy {
    SecurityPolicy::default()
}

/// Parse and validate a production policy from TOML text.
pub fn parse_production_policy(contents: &str) -> Result<SecurityPolicy, String> {
    let policy: SecurityPolicy =
        toml::from_str(contents).map_err(|e| format!("Invalid security TOML: {e}"))?;

    policy.validate_production().map_err(str::to_owned)?;

    Ok(policy)
}

/// Read, parse, and validate a policy file.
pub fn load_production_policy(path: impl AsRef<Path>) -> Result<SecurityPolicy, String> {
    let contents =
        std::fs::read_to_string(path).map_err(|e| format!("Cannot read security policy: {e}"))?;

    parse_production_policy(&contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CONFIG: &str = r#"
selinux_enforcing = true
require_signed_packages = true
require_integrity = true
deny_root_to_apps = true
audit_privileged_actions = true
"#;

    #[test]
    fn production_policy_is_hardened() {
        assert!(production_policy().validate_production().is_ok());
    }

    #[test]
    fn loads_valid_toml_policy() {
        let policy = parse_production_policy(VALID_CONFIG).unwrap();
        assert!(policy.validate_production().is_ok());
    }

    #[test]
    fn rejects_malformed_toml() {
        assert!(parse_production_policy("this is not valid TOML = =").is_err());
    }

    #[test]
    fn rejects_missing_required_fields() {
        assert!(parse_production_policy("selinux_enforcing = true").is_err());
    }

    #[test]
    fn rejects_unknown_fields() {
        let config = format!("{VALID_CONFIG}\nallow_everything = true\n");
        assert!(parse_production_policy(&config).is_err());
    }

    #[test]
    fn rejects_disabled_selinux() {
        let config = VALID_CONFIG.replace("selinux_enforcing = true", "selinux_enforcing = false");
        assert!(parse_production_policy(&config).is_err());
    }

    #[test]
    fn rejects_unsigned_packages() {
        let config = VALID_CONFIG.replace(
            "require_signed_packages = true",
            "require_signed_packages = false",
        );
        assert!(parse_production_policy(&config).is_err());
    }

    #[test]
    fn rejects_disabled_integrity_checks() {
        let config = VALID_CONFIG.replace("require_integrity = true", "require_integrity = false");
        assert!(parse_production_policy(&config).is_err());
    }

    #[test]
    fn rejects_root_access_for_apps() {
        let config = VALID_CONFIG.replace("deny_root_to_apps = true", "deny_root_to_apps = false");
        assert!(parse_production_policy(&config).is_err());
    }

    #[test]
    fn rejects_disabled_privileged_action_auditing() {
        let config = VALID_CONFIG.replace(
            "audit_privileged_actions = true",
            "audit_privileged_actions = false",
        );
        assert!(parse_production_policy(&config).is_err());
    }
}
