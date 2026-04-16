/// Tier of a service gateway SKU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceGatewaySkuTier {
    Regional,
}

impl ServiceGatewaySkuTier {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Regional => "Regional",
        }
    }
}

impl From<ServiceGatewaySkuTier> for serde_json::Value {
    fn from(v: ServiceGatewaySkuTier) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ServiceGatewaySkuTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ServiceGatewaySkuTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Regional" => Ok(Self::Regional),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Regional"])),
        }
    }
}
