/// The type of the group member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupMemberType {
    VirtualNetwork,
    Subnet,
}

impl GroupMemberType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VirtualNetwork => "VirtualNetwork",
            Self::Subnet => "Subnet",
        }
    }
}

impl From<GroupMemberType> for serde_json::Value {
    fn from(v: GroupMemberType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for GroupMemberType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for GroupMemberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "VirtualNetwork" => Ok(Self::VirtualNetwork),
            "Subnet" => Ok(Self::Subnet),
            _ => Err(serde::de::Error::unknown_variant(&s, &["VirtualNetwork", "Subnet"])),
        }
    }
}
