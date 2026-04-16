/// Next hop type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingRuleNextHopType {
    Internet,
    NoNextHop,
    VirtualAppliance,
    VirtualNetworkGateway,
    VnetLocal,
}

impl RoutingRuleNextHopType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Internet => "Internet",
            Self::NoNextHop => "NoNextHop",
            Self::VirtualAppliance => "VirtualAppliance",
            Self::VirtualNetworkGateway => "VirtualNetworkGateway",
            Self::VnetLocal => "VnetLocal",
        }
    }
}

impl From<RoutingRuleNextHopType> for serde_json::Value {
    fn from(v: RoutingRuleNextHopType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for RoutingRuleNextHopType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for RoutingRuleNextHopType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Internet" => Ok(Self::Internet),
            "NoNextHop" => Ok(Self::NoNextHop),
            "VirtualAppliance" => Ok(Self::VirtualAppliance),
            "VirtualNetworkGateway" => Ok(Self::VirtualNetworkGateway),
            "VnetLocal" => Ok(Self::VnetLocal),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Internet", "NoNextHop", "VirtualAppliance", "VirtualNetworkGateway", "VnetLocal"])),
        }
    }
}
