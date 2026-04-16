/// The behavior of the endpoint filter. Currently only 'Include' is supported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionMonitorEndpointFilterType {
    Include,
}

impl ConnectionMonitorEndpointFilterType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Include => "Include",
        }
    }
}

impl From<ConnectionMonitorEndpointFilterType> for serde_json::Value {
    fn from(v: ConnectionMonitorEndpointFilterType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ConnectionMonitorEndpointFilterType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ConnectionMonitorEndpointFilterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Include" => Ok(Self::Include),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Include"])),
        }
    }
}
