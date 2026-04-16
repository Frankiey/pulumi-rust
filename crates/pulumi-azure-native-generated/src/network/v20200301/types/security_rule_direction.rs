/// The direction of the rule. The direction specifies if rule will be evaluated on incoming or outgoing traffic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityRuleDirection {
    Inbound,
    Outbound,
}

impl SecurityRuleDirection {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Inbound => "Inbound",
            Self::Outbound => "Outbound",
        }
    }
}

impl From<SecurityRuleDirection> for serde_json::Value {
    fn from(v: SecurityRuleDirection) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SecurityRuleDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SecurityRuleDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Inbound" => Ok(Self::Inbound),
            "Outbound" => Ok(Self::Outbound),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Inbound", "Outbound"])),
        }
    }
}
