/// The protocol to use in test evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionMonitorTestConfigurationProtocol {
    Tcp,
    Http,
    Icmp,
}

impl ConnectionMonitorTestConfigurationProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "Tcp",
            Self::Http => "Http",
            Self::Icmp => "Icmp",
        }
    }
}

impl From<ConnectionMonitorTestConfigurationProtocol> for serde_json::Value {
    fn from(v: ConnectionMonitorTestConfigurationProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ConnectionMonitorTestConfigurationProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ConnectionMonitorTestConfigurationProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Tcp" => Ok(Self::Tcp),
            "Http" => Ok(Self::Http),
            "Icmp" => Ok(Self::Icmp),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Tcp", "Http", "Icmp"])),
        }
    }
}
