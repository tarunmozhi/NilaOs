use serde::{Deserialize, Serialize};

/// Data classification is explicit so callers can prevent private/secret data
/// from being sent to an online model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataZone {
    Public,
    Private,
    Secret,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    Local,
    Online,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Turn {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub text: String,
    pub zone: DataZone,
    pub online_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Local,
    Online,
    RefuseOnlineForSensitiveData,
}

/// Fail closed: sensitive data is never routed online by this policy helper.
pub fn choose_provider(request: &Request) -> Route {
    if request.zone != DataZone::Public {
        return Route::RefuseOnlineForSensitiveData;
    }
    if request.online_enabled {
        Route::Online
    } else {
        Route::Local
    }
}

/// Small deterministic fallback for connectivity checks and smoke tests.
/// This is not a language model; production answers should come from a model adapter.
pub fn fallback_reply(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return "Please type a question. / ஒரு கேள்வியைத் தட்டச்சு செய்யுங்கள்.".to_string();
    }
    if trimmed.chars().any(|c| ('\u{0B80}'..='\u{0BFF}').contains(&c)) {
        return format!(
            "உங்கள் கேள்வியைப் புரிந்துகொள்ள முயற்சிக்கிறேன்: {}.             முழுமையான பதில்களுக்கு AI model இணைப்பை அமைக்க வேண்டும்.",
            trimmed
        );
    }
    format!(
        "I received your question: {}. Configure a local or approved online model for a full answer.",
        trimmed
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn online_requires_public_data_and_user_opt_in() {
        let public_opted_in = Request {
            text: "What is gravity?".into(),
            zone: DataZone::Public,
            online_enabled: true,
        };
        assert_eq!(choose_provider(&public_opted_in), Route::Online);

        let public_offline = Request {
            online_enabled: false,
            ..public_opted_in.clone()
        };
        assert_eq!(choose_provider(&public_offline), Route::Local);

        for zone in [DataZone::Private, DataZone::Secret] {
            let sensitive = Request {
                zone,
                online_enabled: true,
                ..public_opted_in.clone()
            };
            assert_eq!(choose_provider(&sensitive), Route::RefuseOnlineForSensitiveData);
        }
    }

    #[test]
    fn fallback_handles_empty_input() {
        assert!(fallback_reply("  ").contains("கேள்வியை"));
    }

    #[test]
    fn fallback_recognizes_tamil_input() {
        assert!(fallback_reply("வணக்கம் நிலா").contains("வணக்கம் நிலா"));
    }

    #[test]
    fn fallback_handles_english_input() {
        assert!(fallback_reply("Hello Nila").contains("Hello Nila"));
    }
}
