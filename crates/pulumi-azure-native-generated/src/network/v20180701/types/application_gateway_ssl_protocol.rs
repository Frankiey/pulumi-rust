/// Minimum version of Ssl protocol to be supported on application gateway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewaySslProtocol {
    TLSv10,
    TLSv11,
    TLSv12,
}

impl ApplicationGatewaySslProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TLSv10 => "TLSv1_0",
            Self::TLSv11 => "TLSv1_1",
            Self::TLSv12 => "TLSv1_2",
        }
    }
}

impl From<ApplicationGatewaySslProtocol> for serde_json::Value {
    fn from(v: ApplicationGatewaySslProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewaySslProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewaySslProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "TLSv1_0" => Ok(Self::TLSv10),
            "TLSv1_1" => Ok(Self::TLSv11),
            "TLSv1_2" => Ok(Self::TLSv12),
            _ => Err(serde::de::Error::unknown_variant(&s, &["TLSv1_0", "TLSv1_1", "TLSv1_2"])),
        }
    }
}
