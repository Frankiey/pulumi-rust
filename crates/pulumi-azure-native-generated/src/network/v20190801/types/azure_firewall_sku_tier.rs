/// Tier of an Azure Firewall.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AzureFirewallSkuTier {
    Standard,
}

impl AzureFirewallSkuTier {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
        }
    }
}

impl From<AzureFirewallSkuTier> for serde_json::Value {
    fn from(v: AzureFirewallSkuTier) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AzureFirewallSkuTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AzureFirewallSkuTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Standard" => Ok(Self::Standard),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Standard"])),
        }
    }
}
