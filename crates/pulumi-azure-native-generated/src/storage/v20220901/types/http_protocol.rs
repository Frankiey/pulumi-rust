/// The protocol permitted for a request made with the account SAS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpProtocol {
    HttpsHttp,
    Https,
}

impl HttpProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::HttpsHttp => "https,http",
            Self::Https => "https",
        }
    }
}

impl From<HttpProtocol> for serde_json::Value {
    fn from(v: HttpProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for HttpProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for HttpProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "https,http" => Ok(Self::HttpsHttp),
            "https" => Ok(Self::Https),
            _ => Err(serde::de::Error::unknown_variant(&s, &["https,http", "https"])),
        }
    }
}
