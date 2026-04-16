/// The reference to the transport protocol used by the load balancing rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportProtocol {
    Udp,
    Tcp,
    All,
    Quic,
}

impl TransportProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Udp => "Udp",
            Self::Tcp => "Tcp",
            Self::All => "All",
            Self::Quic => "Quic",
        }
    }
}

impl From<TransportProtocol> for serde_json::Value {
    fn from(v: TransportProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for TransportProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for TransportProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Udp" => Ok(Self::Udp),
            "Tcp" => Ok(Self::Tcp),
            "All" => Ok(Self::All),
            "Quic" => Ok(Self::Quic),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Udp", "Tcp", "All", "Quic"])),
        }
    }
}
