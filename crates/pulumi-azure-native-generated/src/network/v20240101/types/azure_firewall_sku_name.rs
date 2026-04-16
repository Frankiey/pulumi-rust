/// Name of an Azure Firewall SKU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AzureFirewallSkuName {
    AZFWVNet,
    AZFWHub,
}

impl AzureFirewallSkuName {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AZFWVNet => "AZFW_VNet",
            Self::AZFWHub => "AZFW_Hub",
        }
    }
}

impl From<AzureFirewallSkuName> for serde_json::Value {
    fn from(v: AzureFirewallSkuName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AzureFirewallSkuName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AzureFirewallSkuName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AZFW_VNet" => Ok(Self::AZFWVNet),
            "AZFW_Hub" => Ok(Self::AZFWHub),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AZFW_VNet", "AZFW_Hub"])),
        }
    }
}
