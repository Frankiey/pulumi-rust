/// Verify client certificate revocation status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewayClientRevocationOptions {
    None,
    OCSP,
}

impl ApplicationGatewayClientRevocationOptions {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::OCSP => "OCSP",
        }
    }
}

impl From<ApplicationGatewayClientRevocationOptions> for serde_json::Value {
    fn from(v: ApplicationGatewayClientRevocationOptions) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewayClientRevocationOptions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewayClientRevocationOptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "OCSP" => Ok(Self::OCSP),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "OCSP"])),
        }
    }
}
