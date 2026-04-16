/// Indicates the access allowed for this particular rule
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityConfigurationRuleAccess {
    Allow,
    Deny,
    AlwaysAllow,
}

impl SecurityConfigurationRuleAccess {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Allow => "Allow",
            Self::Deny => "Deny",
            Self::AlwaysAllow => "AlwaysAllow",
        }
    }
}

impl From<SecurityConfigurationRuleAccess> for serde_json::Value {
    fn from(v: SecurityConfigurationRuleAccess) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SecurityConfigurationRuleAccess {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SecurityConfigurationRuleAccess {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Allow" => Ok(Self::Allow),
            "Deny" => Ok(Self::Deny),
            "AlwaysAllow" => Ok(Self::AlwaysAllow),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Allow", "Deny", "AlwaysAllow"])),
        }
    }
}
