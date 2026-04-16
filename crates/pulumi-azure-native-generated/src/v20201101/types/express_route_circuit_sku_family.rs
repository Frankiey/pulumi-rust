/// The family of the SKU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressRouteCircuitSkuFamily {
    UnlimitedData,
    MeteredData,
}

impl ExpressRouteCircuitSkuFamily {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnlimitedData => "UnlimitedData",
            Self::MeteredData => "MeteredData",
        }
    }
}

impl From<ExpressRouteCircuitSkuFamily> for serde_json::Value {
    fn from(v: ExpressRouteCircuitSkuFamily) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExpressRouteCircuitSkuFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpressRouteCircuitSkuFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "UnlimitedData" => Ok(Self::UnlimitedData),
            "MeteredData" => Ok(Self::MeteredData),
            _ => Err(serde::de::Error::unknown_variant(&s, &["UnlimitedData", "MeteredData"])),
        }
    }
}
