/// Connectivity topology type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectivityTopology {
    HubAndSpoke,
    Mesh,
}

impl ConnectivityTopology {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::HubAndSpoke => "HubAndSpoke",
            Self::Mesh => "Mesh",
        }
    }
}

impl From<ConnectivityTopology> for serde_json::Value {
    fn from(v: ConnectivityTopology) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ConnectivityTopology {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ConnectivityTopology {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "HubAndSpoke" => Ok(Self::HubAndSpoke),
            "Mesh" => Ok(Self::Mesh),
            _ => Err(serde::de::Error::unknown_variant(&s, &["HubAndSpoke", "Mesh"])),
        }
    }
}
