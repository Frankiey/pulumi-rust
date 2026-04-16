/// Behavior to handle overlapped IP address space among members of the connected group of the connectivity configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectedGroupAddressOverlap {
    /// Default. Allows connected group members to have overlapping IP address space.
    Allowed,
    /// Strictly disallows connected group members from having overlapping IP address space. Prevents the addition of a virtual network with overlapping address to the connected group, blocks peering between a virtual network and a connected group member if any connected group member has an overlapping range, and restricts address space modifications that would introduce overlap.
    Disallowed,
}

impl ConnectedGroupAddressOverlap {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Allowed => "Allowed",
            Self::Disallowed => "Disallowed",
        }
    }
}

impl From<ConnectedGroupAddressOverlap> for serde_json::Value {
    fn from(v: ConnectedGroupAddressOverlap) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ConnectedGroupAddressOverlap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ConnectedGroupAddressOverlap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Allowed" => Ok(Self::Allowed),
            "Disallowed" => Ok(Self::Disallowed),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Allowed", "Disallowed"])),
        }
    }
}
