/// Describes the override action to be applied when rule matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    AnomalyScoring,
    Allow,
    Block,
    Log,
    JSChallenge,
    CAPTCHA,
}

impl ActionType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AnomalyScoring => "AnomalyScoring",
            Self::Allow => "Allow",
            Self::Block => "Block",
            Self::Log => "Log",
            Self::JSChallenge => "JSChallenge",
            Self::CAPTCHA => "CAPTCHA",
        }
    }
}

impl From<ActionType> for serde_json::Value {
    fn from(v: ActionType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ActionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AnomalyScoring" => Ok(Self::AnomalyScoring),
            "Allow" => Ok(Self::Allow),
            "Block" => Ok(Self::Block),
            "Log" => Ok(Self::Log),
            "JSChallenge" => Ok(Self::JSChallenge),
            "CAPTCHA" => Ok(Self::CAPTCHA),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AnomalyScoring", "Allow", "Block", "Log", "JSChallenge", "CAPTCHA"])),
        }
    }
}
