/// The type of Azure hop the packet should be sent to. Possible values are: 'VirtualNetworkGateway', 'VnetLocal', 'Internet', 'VirtualAppliance', and 'None'
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteNextHopType {
    VirtualNetworkGateway,
    VnetLocal,
    Internet,
    VirtualAppliance,
    None,
}

impl RouteNextHopType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VirtualNetworkGateway => "VirtualNetworkGateway",
            Self::VnetLocal => "VnetLocal",
            Self::Internet => "Internet",
            Self::VirtualAppliance => "VirtualAppliance",
            Self::None => "None",
        }
    }
}

impl From<RouteNextHopType> for serde_json::Value {
    fn from(v: RouteNextHopType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for RouteNextHopType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for RouteNextHopType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "VirtualNetworkGateway" => Ok(Self::VirtualNetworkGateway),
            "VnetLocal" => Ok(Self::VnetLocal),
            "Internet" => Ok(Self::Internet),
            "VirtualAppliance" => Ok(Self::VirtualAppliance),
            "None" => Ok(Self::None),
            _ => Err(serde::de::Error::unknown_variant(&s, &["VirtualNetworkGateway", "VnetLocal", "Internet", "VirtualAppliance", "None"])),
        }
    }
}
