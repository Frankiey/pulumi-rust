/// Direction that specifies whether the access rules is inbound/outbound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessRuleDirection {
    Inbound,
    Outbound,
}

impl AccessRuleDirection {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Inbound => "Inbound",
            Self::Outbound => "Outbound",
        }
    }
}

impl From<AccessRuleDirection> for serde_json::Value {
    fn from(v: AccessRuleDirection) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AccessRuleDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AccessRuleDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Inbound" => Ok(Self::Inbound),
            "Outbound" => Ok(Self::Outbound),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Inbound", "Outbound"])),
        }
    }
}
