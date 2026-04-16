/// The preferred IP version to use in test evaluation. The connection monitor may choose to use a different version depending on other parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreferredIPVersion {
    IPv4,
    IPv6,
}

impl PreferredIPVersion {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IPv4 => "IPv4",
            Self::IPv6 => "IPv6",
        }
    }
}

impl From<PreferredIPVersion> for serde_json::Value {
    fn from(v: PreferredIPVersion) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PreferredIPVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PreferredIPVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "IPv4" => Ok(Self::IPv4),
            "IPv6" => Ok(Self::IPv6),
            _ => Err(serde::de::Error::unknown_variant(&s, &["IPv4", "IPv6"])),
        }
    }
}
