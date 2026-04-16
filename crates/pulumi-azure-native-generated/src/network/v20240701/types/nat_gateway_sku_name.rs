/// Name of Nat Gateway SKU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NatGatewaySkuName {
    Standard,
    StandardV2,
}

impl NatGatewaySkuName {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::StandardV2 => "StandardV2",
        }
    }
}

impl From<NatGatewaySkuName> for serde_json::Value {
    fn from(v: NatGatewaySkuName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for NatGatewaySkuName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for NatGatewaySkuName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Standard" => Ok(Self::Standard),
            "StandardV2" => Ok(Self::StandardV2),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Standard", "StandardV2"])),
        }
    }
}
