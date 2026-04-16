/// AdvertisedPublicPrefixState of the Peering resource. Possible values are 'NotConfigured', 'Configuring', 'Configured', and 'ValidationNeeded'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressRouteCircuitPeeringAdvertisedPublicPrefixState {
    NotConfigured,
    Configuring,
    Configured,
    ValidationNeeded,
}

impl ExpressRouteCircuitPeeringAdvertisedPublicPrefixState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotConfigured => "NotConfigured",
            Self::Configuring => "Configuring",
            Self::Configured => "Configured",
            Self::ValidationNeeded => "ValidationNeeded",
        }
    }
}

impl From<ExpressRouteCircuitPeeringAdvertisedPublicPrefixState> for serde_json::Value {
    fn from(v: ExpressRouteCircuitPeeringAdvertisedPublicPrefixState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExpressRouteCircuitPeeringAdvertisedPublicPrefixState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpressRouteCircuitPeeringAdvertisedPublicPrefixState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "NotConfigured" => Ok(Self::NotConfigured),
            "Configuring" => Ok(Self::Configuring),
            "Configured" => Ok(Self::Configured),
            "ValidationNeeded" => Ok(Self::ValidationNeeded),
            _ => Err(serde::de::Error::unknown_variant(&s, &["NotConfigured", "Configuring", "Configured", "ValidationNeeded"])),
        }
    }
}
