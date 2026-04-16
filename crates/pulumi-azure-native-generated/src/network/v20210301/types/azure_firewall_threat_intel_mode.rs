/// The operation mode for Threat Intelligence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AzureFirewallThreatIntelMode {
    Alert,
    Deny,
    Off,
}

impl AzureFirewallThreatIntelMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alert => "Alert",
            Self::Deny => "Deny",
            Self::Off => "Off",
        }
    }
}

impl From<AzureFirewallThreatIntelMode> for serde_json::Value {
    fn from(v: AzureFirewallThreatIntelMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AzureFirewallThreatIntelMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AzureFirewallThreatIntelMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Alert" => Ok(Self::Alert),
            "Deny" => Ok(Self::Deny),
            "Off" => Ok(Self::Off),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Alert", "Deny", "Off"])),
        }
    }
}
