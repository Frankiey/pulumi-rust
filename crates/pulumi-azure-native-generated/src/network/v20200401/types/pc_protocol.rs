/// Protocol to be filtered on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PcProtocol {
    TCP,
    UDP,
    Any,
}

impl PcProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TCP => "TCP",
            Self::UDP => "UDP",
            Self::Any => "Any",
        }
    }
}

impl From<PcProtocol> for serde_json::Value {
    fn from(v: PcProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PcProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PcProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "TCP" => Ok(Self::TCP),
            "UDP" => Ok(Self::UDP),
            "Any" => Ok(Self::Any),
            _ => Err(serde::de::Error::unknown_variant(&s, &["TCP", "UDP", "Any"])),
        }
    }
}
