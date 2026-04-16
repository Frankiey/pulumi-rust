/// The hubRoutingPreference of this VirtualHub.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HubRoutingPreference {
    ExpressRoute,
    VpnGateway,
    ASPath,
}

impl HubRoutingPreference {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExpressRoute => "ExpressRoute",
            Self::VpnGateway => "VpnGateway",
            Self::ASPath => "ASPath",
        }
    }
}

impl From<HubRoutingPreference> for serde_json::Value {
    fn from(v: HubRoutingPreference) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for HubRoutingPreference {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for HubRoutingPreference {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ExpressRoute" => Ok(Self::ExpressRoute),
            "VpnGateway" => Ok(Self::VpnGateway),
            "ASPath" => Ok(Self::ASPath),
            _ => Err(serde::de::Error::unknown_variant(&s, &["ExpressRoute", "VpnGateway", "ASPath"])),
        }
    }
}
