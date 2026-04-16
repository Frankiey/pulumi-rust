/// Network protocol this resource applies to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkProtocol {
    Any,
    TCP,
    UDP,
    ICMP,
}

impl NetworkProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Any => "Any",
            Self::TCP => "TCP",
            Self::UDP => "UDP",
            Self::ICMP => "ICMP",
        }
    }
}

impl From<NetworkProtocol> for serde_json::Value {
    fn from(v: NetworkProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for NetworkProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for NetworkProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Any" => Ok(Self::Any),
            "TCP" => Ok(Self::TCP),
            "UDP" => Ok(Self::UDP),
            "ICMP" => Ok(Self::ICMP),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Any", "TCP", "UDP", "ICMP"])),
        }
    }
}
