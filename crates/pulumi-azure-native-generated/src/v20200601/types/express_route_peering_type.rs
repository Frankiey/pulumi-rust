/// The peering type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressRoutePeeringType {
    AzurePublicPeering,
    AzurePrivatePeering,
    MicrosoftPeering,
}

impl ExpressRoutePeeringType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AzurePublicPeering => "AzurePublicPeering",
            Self::AzurePrivatePeering => "AzurePrivatePeering",
            Self::MicrosoftPeering => "MicrosoftPeering",
        }
    }
}

impl From<ExpressRoutePeeringType> for serde_json::Value {
    fn from(v: ExpressRoutePeeringType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExpressRoutePeeringType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpressRoutePeeringType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AzurePublicPeering" => Ok(Self::AzurePublicPeering),
            "AzurePrivatePeering" => Ok(Self::AzurePrivatePeering),
            "MicrosoftPeering" => Ok(Self::MicrosoftPeering),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AzurePublicPeering", "AzurePrivatePeering", "MicrosoftPeering"])),
        }
    }
}
