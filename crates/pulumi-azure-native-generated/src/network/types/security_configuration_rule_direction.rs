/// Indicates if the traffic matched against the rule in inbound or outbound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityConfigurationRuleDirection {
    Inbound,
    Outbound,
}

impl SecurityConfigurationRuleDirection {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Inbound => "Inbound",
            Self::Outbound => "Outbound",
        }
    }
}

impl From<SecurityConfigurationRuleDirection> for serde_json::Value {
    fn from(v: SecurityConfigurationRuleDirection) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SecurityConfigurationRuleDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SecurityConfigurationRuleDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Inbound" => Ok(Self::Inbound),
            "Outbound" => Ok(Self::Outbound),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Inbound", "Outbound"])),
        }
    }
}
