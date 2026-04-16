/// Status code of the application gateway customer error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewayCustomErrorStatusCode {
    HttpStatus403,
    HttpStatus502,
}

impl ApplicationGatewayCustomErrorStatusCode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::HttpStatus403 => "HttpStatus403",
            Self::HttpStatus502 => "HttpStatus502",
        }
    }
}

impl From<ApplicationGatewayCustomErrorStatusCode> for serde_json::Value {
    fn from(v: ApplicationGatewayCustomErrorStatusCode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewayCustomErrorStatusCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewayCustomErrorStatusCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "HttpStatus403" => Ok(Self::HttpStatus403),
            "HttpStatus502" => Ok(Self::HttpStatus502),
            _ => Err(serde::de::Error::unknown_variant(&s, &["HttpStatus403", "HttpStatus502"])),
        }
    }
}
