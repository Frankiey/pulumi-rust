/// The status of the virtual network peering. Possible values are 'Initiated', 'Connected', and 'Disconnected'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkPeeringState {
    Initiated,
    Connected,
    Disconnected,
}

impl VirtualNetworkPeeringState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Initiated => "Initiated",
            Self::Connected => "Connected",
            Self::Disconnected => "Disconnected",
        }
    }
}

impl From<VirtualNetworkPeeringState> for serde_json::Value {
    fn from(v: VirtualNetworkPeeringState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkPeeringState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkPeeringState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Initiated" => Ok(Self::Initiated),
            "Connected" => Ok(Self::Connected),
            "Disconnected" => Ok(Self::Disconnected),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Initiated", "Connected", "Disconnected"])),
        }
    }
}
