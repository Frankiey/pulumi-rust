/// The protocol of the end point. If 'Tcp' is specified, a received ACK is required for the probe to be successful. If 'Http' or 'Https' is specified, a 200 OK response from the specifies URI is required for the probe to be successful.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeProtocol {
    Http,
    Tcp,
    Https,
}

impl ProbeProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Http => "Http",
            Self::Tcp => "Tcp",
            Self::Https => "Https",
        }
    }
}

impl From<ProbeProtocol> for serde_json::Value {
    fn from(v: ProbeProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ProbeProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ProbeProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Http" => Ok(Self::Http),
            "Tcp" => Ok(Self::Tcp),
            "Https" => Ok(Self::Https),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Http", "Tcp", "Https"])),
        }
    }
}
