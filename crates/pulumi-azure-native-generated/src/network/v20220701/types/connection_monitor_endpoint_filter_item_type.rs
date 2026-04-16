/// The type of item included in the filter. Currently only 'AgentAddress' is supported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionMonitorEndpointFilterItemType {
    AgentAddress,
}

impl ConnectionMonitorEndpointFilterItemType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AgentAddress => "AgentAddress",
        }
    }
}

impl From<ConnectionMonitorEndpointFilterItemType> for serde_json::Value {
    fn from(v: ConnectionMonitorEndpointFilterItemType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ConnectionMonitorEndpointFilterItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ConnectionMonitorEndpointFilterItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AgentAddress" => Ok(Self::AgentAddress),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AgentAddress"])),
        }
    }
}
