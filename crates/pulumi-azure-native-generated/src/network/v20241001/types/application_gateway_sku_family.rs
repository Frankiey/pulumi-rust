/// Family of an application gateway SKU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewaySkuFamily {
    Generation1,
    Generation2,
}

impl ApplicationGatewaySkuFamily {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Generation1 => "Generation_1",
            Self::Generation2 => "Generation_2",
        }
    }
}

impl From<ApplicationGatewaySkuFamily> for serde_json::Value {
    fn from(v: ApplicationGatewaySkuFamily) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewaySkuFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewaySkuFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Generation_1" => Ok(Self::Generation1),
            "Generation_2" => Ok(Self::Generation2),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Generation_1", "Generation_2"])),
        }
    }
}
