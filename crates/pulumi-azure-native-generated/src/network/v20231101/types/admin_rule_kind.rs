/// Whether the rule is custom or default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminRuleKind {
    Custom,
    Default,
}

impl AdminRuleKind {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Custom => "Custom",
            Self::Default => "Default",
        }
    }
}

impl From<AdminRuleKind> for serde_json::Value {
    fn from(v: AdminRuleKind) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AdminRuleKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AdminRuleKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Custom" => Ok(Self::Custom),
            "Default" => Ok(Self::Default),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Custom", "Default"])),
        }
    }
}
