/// Status code of the application gateway custom error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewayCustomErrorStatusCode {
    HttpStatus400,
    HttpStatus403,
    HttpStatus404,
    HttpStatus405,
    HttpStatus408,
    HttpStatus499,
    HttpStatus500,
    HttpStatus502,
    HttpStatus503,
    HttpStatus504,
}

impl ApplicationGatewayCustomErrorStatusCode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::HttpStatus400 => "HttpStatus400",
            Self::HttpStatus403 => "HttpStatus403",
            Self::HttpStatus404 => "HttpStatus404",
            Self::HttpStatus405 => "HttpStatus405",
            Self::HttpStatus408 => "HttpStatus408",
            Self::HttpStatus499 => "HttpStatus499",
            Self::HttpStatus500 => "HttpStatus500",
            Self::HttpStatus502 => "HttpStatus502",
            Self::HttpStatus503 => "HttpStatus503",
            Self::HttpStatus504 => "HttpStatus504",
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
            "HttpStatus400" => Ok(Self::HttpStatus400),
            "HttpStatus403" => Ok(Self::HttpStatus403),
            "HttpStatus404" => Ok(Self::HttpStatus404),
            "HttpStatus405" => Ok(Self::HttpStatus405),
            "HttpStatus408" => Ok(Self::HttpStatus408),
            "HttpStatus499" => Ok(Self::HttpStatus499),
            "HttpStatus500" => Ok(Self::HttpStatus500),
            "HttpStatus502" => Ok(Self::HttpStatus502),
            "HttpStatus503" => Ok(Self::HttpStatus503),
            "HttpStatus504" => Ok(Self::HttpStatus504),
            _ => Err(serde::de::Error::unknown_variant(&s, &["HttpStatus400", "HttpStatus403", "HttpStatus404", "HttpStatus405", "HttpStatus408", "HttpStatus499", "HttpStatus500", "HttpStatus502", "HttpStatus503", "HttpStatus504"])),
        }
    }
}
