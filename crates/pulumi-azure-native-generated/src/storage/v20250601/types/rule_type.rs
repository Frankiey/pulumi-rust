/// The valid value is Lifecycle
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleType {
    Lifecycle,
}

impl RuleType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lifecycle => "Lifecycle",
        }
    }
}

impl From<RuleType> for serde_json::Value {
    fn from(v: RuleType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for RuleType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for RuleType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Lifecycle" => Ok(Self::Lifecycle),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Lifecycle"])),
        }
    }
}
