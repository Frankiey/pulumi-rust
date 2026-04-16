/// Gateway connection authentication type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionAuthenticationType {
    /// Pre-shared key authentication method for VPN gateway connections.
    PSK,
    /// Certificate-based authentication method for VPN gateway connections.
    Certificate,
}

impl ConnectionAuthenticationType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PSK => "PSK",
            Self::Certificate => "Certificate",
        }
    }
}

impl From<ConnectionAuthenticationType> for serde_json::Value {
    fn from(v: ConnectionAuthenticationType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ConnectionAuthenticationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ConnectionAuthenticationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "PSK" => Ok(Self::PSK),
            "Certificate" => Ok(Self::Certificate),
            _ => Err(serde::de::Error::unknown_variant(&s, &["PSK", "Certificate"])),
        }
    }
}
