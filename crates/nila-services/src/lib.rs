use nila_security::SecurityPolicy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub domain: String,
    pub privileged: bool,
}

pub fn default_services() -> Vec<Service> {
    vec![
        Service {
            name: "nilad-security".into(),
            domain: "u:r:nilad_security:s0".into(),
            privileged: true,
        },
        Service {
            name: "nilad-resource".into(),
            domain: "u:r:nilad_resource:s0".into(),
            privileged: true,
        },
        Service {
            name: "nilad-storage".into(),
            domain: "u:r:nilad_storage:s0".into(),
            privileged: true,
        },
        Service {
            name: "nilad-net".into(),
            domain: "u:r:nilad_net:s0".into(),
            privileged: true,
        },
        Service {
            name: "nilad-sys".into(),
            domain: "u:r:nilad_sys:s0".into(),
            privileged: true,
        },
        Service {
            name: "mfr-sandbox".into(),
            domain: "u:r:mfr_sandbox:s0".into(),
            privileged: false,
        },
        Service {
            name: "nila-browser".into(),
            domain: "u:r:nila_browser:s0".into(),
            privileged: false,
        },
        Service {
            name: "nila-assistant".into(),
            domain: "u:r:nila_assistant_sandbox:s0".into(),
            privileged: false,
        },
    ]
}

/// Return the service list only when the production policy is valid.
/// This validates configuration; it does not launch services.
pub fn validated_services(policy: &SecurityPolicy) -> Result<Vec<Service>, String> {
    policy.validate_production().map_err(str::to_owned)?;

    Ok(default_services())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_services_exist() {
        let services = default_services();
        assert!(services.iter().any(|s| s.name == "nilad-security"));
        assert!(services.iter().any(|s| s.name == "mfr-sandbox"));
    }

    #[test]
    fn valid_policy_allows_service_list() {
        let policy = SecurityPolicy::default();
        assert!(validated_services(&policy).is_ok());
    }

    #[test]
    fn weakened_policy_rejects_service_list() {
        let mut policy = SecurityPolicy::default();
        policy.require_signed_packages = false;
        assert!(validated_services(&policy).is_err());
    }

    #[test]
    fn application_services_are_unprivileged() {
        let services = default_services();

        for name in ["mfr-sandbox", "nila-browser", "nila-assistant"] {
            let service = services.iter().find(|s| s.name == name).unwrap();
            assert!(!service.privileged, "{name} must remain unprivileged");
        }
    }
}
