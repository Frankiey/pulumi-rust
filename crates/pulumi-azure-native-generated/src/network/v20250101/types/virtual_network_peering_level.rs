/// The peering sync status of the virtual network peering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualNetworkPeeringLevel {
    FullyInSync,
    RemoteNotInSync,
    LocalNotInSync,
    LocalAndRemoteNotInSync,
}

impl VirtualNetworkPeeringLevel {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FullyInSync => "FullyInSync",
            Self::RemoteNotInSync => "RemoteNotInSync",
            Self::LocalNotInSync => "LocalNotInSync",
            Self::LocalAndRemoteNotInSync => "LocalAndRemoteNotInSync",
        }
    }
}

impl From<VirtualNetworkPeeringLevel> for serde_json::Value {
    fn from(v: VirtualNetworkPeeringLevel) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VirtualNetworkPeeringLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VirtualNetworkPeeringLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "FullyInSync" => Ok(Self::FullyInSync),
            "RemoteNotInSync" => Ok(Self::RemoteNotInSync),
            "LocalNotInSync" => Ok(Self::LocalNotInSync),
            "LocalAndRemoteNotInSync" => Ok(Self::LocalAndRemoteNotInSync),
            _ => Err(serde::de::Error::unknown_variant(&s, &["FullyInSync", "RemoteNotInSync", "LocalNotInSync", "LocalAndRemoteNotInSync"])),
        }
    }
}
