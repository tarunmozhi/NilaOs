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

/// Identity requesting an operation. These are policy labels, not OS credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Actor {
    Owner,
    Assistant,
    Application,
    SystemService,
}

/// A narrowly-scoped capability requested from the Nila permission broker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    ReadDeviceInfo,
    ChangeNetwork,
    AccessMicrophone,
    HandleCalls,
    AccessAccountSession,
    AccessSecrets,
    ManagePackages,
    InstallSystemUpdate,
    EnterRecovery,
}

impl Capability {
    fn always_owner_only(self) -> bool {
        matches!(
            self,
            Self::AccessSecrets | Self::ManagePackages | Self::InstallSystemUpdate | Self::EnterRecovery
        )
    }

    fn requires_confirmation(self) -> bool {
        matches!(
            self,
            Self::ChangeNetwork
                | Self::AccessMicrophone
                | Self::HandleCalls
                | Self::AccessAccountSession
                | Self::ManagePackages
                | Self::InstallSystemUpdate
                | Self::EnterRecovery
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionRequest {
    pub actor: Actor,
    pub capability: Capability,
    /// A grant recorded by the OS permission broker; never trust a UI checkbox alone.
    pub grant_present: bool,
    /// A fresh confirmation for this operation, when required by policy.
    pub confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub reason: &'static str,
}

impl PolicyDecision {
    fn deny(reason: &'static str) -> Self {
        Self { allowed: false, reason }
    }

    fn allow(reason: &'static str) -> Self {
        Self { allowed: true, reason }
    }
}

/// Evaluate a request conservatively. This is a policy-library decision only;
/// a real OS broker must authenticate callers and enforce the decision at the
/// service/kernel boundary before any operation is performed.
pub fn evaluate_action(request: ActionRequest) -> PolicyDecision {
    if request.actor == Actor::Application && request.capability.always_owner_only() {
        return PolicyDecision::deny("capability is owner-only");
    }

    if request.actor == Actor::Assistant && request.capability.always_owner_only() {
        return PolicyDecision::deny("assistant cannot request owner-only capability");
    }

    if request.actor == Actor::SystemService && !request.grant_present {
        return PolicyDecision::deny("system service request lacks an explicit broker grant");
    }

    if matches!(request.actor, Actor::Assistant | Actor::Application) && !request.grant_present {
        return PolicyDecision::deny("no explicit permission grant");
    }

    if request.capability.requires_confirmation() && !request.confirmed {
        return PolicyDecision::deny("fresh user confirmation required");
    }

    PolicyDecision::allow("policy checks passed; OS enforcement is still required")
}

/// Minimal audit metadata. Deliberately excludes credentials, arguments, and payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEvent {
    pub actor: Actor,
    pub capability: Capability,
    pub allowed: bool,
    pub reason: String,
}

impl From<(ActionRequest, PolicyDecision)> for AuditEvent {
    fn from((request, decision): (ActionRequest, PolicyDecision)) -> Self {
        Self {
            actor: request.actor,
            capability: request.capability,
            allowed: decision.allowed,
            reason: decision.reason.to_owned(),
        }
    }
}

#[cfg(test)]
mod permission_tests {
    use super::*;

    fn request(actor: Actor, capability: Capability, grant_present: bool, confirmed: bool) -> ActionRequest {
        ActionRequest { actor, capability, grant_present, confirmed }
    }

    #[test]
    fn denies_assistant_access_to_secrets_even_with_grant() {
        let decision = evaluate_action(request(Actor::Assistant, Capability::AccessSecrets, true, true));
        assert!(!decision.allowed);
    }

    #[test]
    fn denies_application_package_management() {
        let decision = evaluate_action(request(Actor::Application, Capability::ManagePackages, true, true));
        assert!(!decision.allowed);
    }

    #[test]
    fn denies_missing_grant() {
        let decision = evaluate_action(request(Actor::Assistant, Capability::ReadDeviceInfo, false, false));
        assert!(!decision.allowed);
    }

    #[test]
    fn requires_confirmation_for_calls() {
        let decision = evaluate_action(request(Actor::Assistant, Capability::HandleCalls, true, false));
        assert!(!decision.allowed);
    }

    #[test]
    fn allows_granted_and_confirmed_call_request() {
        let decision = evaluate_action(request(Actor::Assistant, Capability::HandleCalls, true, true));
        assert!(decision.allowed);
    }

    #[test]
    fn ordinary_read_only_app_capability_needs_grant_but_not_confirmation() {
        let denied = evaluate_action(request(Actor::Application, Capability::ReadDeviceInfo, false, false));
        let allowed = evaluate_action(request(Actor::Application, Capability::ReadDeviceInfo, true, false));
        assert!(!denied.allowed);
        assert!(allowed.allowed);
    }

    #[test]
    fn audit_event_contains_decision_without_secret_payload() {
        let req = request(Actor::Assistant, Capability::AccessAccountSession, true, false);
        let decision = evaluate_action(req);
        let event = AuditEvent::from((req, decision));
        assert!(!event.allowed);
        assert_eq!(event.capability, Capability::AccessAccountSession);
        assert!(!event.reason.contains("password"));
    }
}
