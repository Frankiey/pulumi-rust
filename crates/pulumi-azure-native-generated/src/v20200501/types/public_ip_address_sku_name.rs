/// Name of a public IP address SKU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicIPAddressSkuName {
    Basic,
    Standard,
}

impl PublicIPAddressSkuName {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Standard => "Standard",
        }
    }
}

impl From<PublicIPAddressSkuName> for serde_json::Value {
    fn from(v: PublicIPAddressSkuName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PublicIPAddressSkuName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PublicIPAddressSkuName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Basic" => Ok(Self::Basic),
            "Standard" => Ok(Self::Standard),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Basic", "Standard"])),
        }
    }
}
