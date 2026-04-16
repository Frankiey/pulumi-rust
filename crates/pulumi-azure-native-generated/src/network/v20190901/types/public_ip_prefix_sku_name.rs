/// Name of a public IP prefix SKU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicIPPrefixSkuName {
    Standard,
}

impl PublicIPPrefixSkuName {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
        }
    }
}

impl From<PublicIPPrefixSkuName> for serde_json::Value {
    fn from(v: PublicIPPrefixSkuName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PublicIPPrefixSkuName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PublicIPPrefixSkuName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Standard" => Ok(Self::Standard),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Standard"])),
        }
    }
}
