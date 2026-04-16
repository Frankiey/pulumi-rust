/// Protocol type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AzureFirewallApplicationRuleProtocolType {
    Http,
    Https,
}

impl AzureFirewallApplicationRuleProtocolType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Http => "Http",
            Self::Https => "Https",
        }
    }
}

impl From<AzureFirewallApplicationRuleProtocolType> for serde_json::Value {
    fn from(v: AzureFirewallApplicationRuleProtocolType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AzureFirewallApplicationRuleProtocolType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AzureFirewallApplicationRuleProtocolType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Http" => Ok(Self::Http),
            "Https" => Ok(Self::Https),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Http", "Https"])),
        }
    }
}
