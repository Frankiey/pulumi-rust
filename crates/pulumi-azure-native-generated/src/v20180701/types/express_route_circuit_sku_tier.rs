/// The tier of the SKU. Possible values are 'Standard' and 'Premium'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressRouteCircuitSkuTier {
    Standard,
    Premium,
}

impl ExpressRouteCircuitSkuTier {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::Premium => "Premium",
        }
    }
}

impl From<ExpressRouteCircuitSkuTier> for serde_json::Value {
    fn from(v: ExpressRouteCircuitSkuTier) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExpressRouteCircuitSkuTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpressRouteCircuitSkuTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Standard" => Ok(Self::Standard),
            "Premium" => Ok(Self::Premium),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Standard", "Premium"])),
        }
    }
}
