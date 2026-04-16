/// Available from Api-Version 2016-03-30 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IPVersion {
    IPv4,
    IPv6,
}

impl IPVersion {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IPv4 => "IPv4",
            Self::IPv6 => "IPv6",
        }
    }
}

impl From<IPVersion> for serde_json::Value {
    fn from(v: IPVersion) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for IPVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for IPVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "IPv4" => Ok(Self::IPv4),
            "IPv6" => Ok(Self::IPv6),
            _ => Err(serde::de::Error::unknown_variant(&s, &["IPv4", "IPv6"])),
        }
    }
}
