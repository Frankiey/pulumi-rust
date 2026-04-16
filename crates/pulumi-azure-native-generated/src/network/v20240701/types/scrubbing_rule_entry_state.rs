/// Defines the state of log scrubbing rule. Default value is Enabled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrubbingRuleEntryState {
    Enabled,
    Disabled,
}

impl ScrubbingRuleEntryState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}

impl From<ScrubbingRuleEntryState> for serde_json::Value {
    fn from(v: ScrubbingRuleEntryState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ScrubbingRuleEntryState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ScrubbingRuleEntryState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Enabled", "Disabled"])),
        }
    }
}
