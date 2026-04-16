/// Allows you to specify the type of endpoint. Set this to AzureDNSZone to create a large number of accounts in a single subscription, which creates accounts in an Azure DNS Zone and the endpoint URL will have an alphanumeric DNS Zone identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsEndpointType {
    Standard,
    AzureDnsZone,
}

impl DnsEndpointType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::AzureDnsZone => "AzureDnsZone",
        }
    }
}

impl From<DnsEndpointType> for serde_json::Value {
    fn from(v: DnsEndpointType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DnsEndpointType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DnsEndpointType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Standard" => Ok(Self::Standard),
            "AzureDnsZone" => Ok(Self::AzureDnsZone),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Standard", "AzureDnsZone"])),
        }
    }
}
