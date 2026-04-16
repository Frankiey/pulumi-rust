#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowedMethods {
    DELETE,
    GET,
    HEAD,
    MERGE,
    POST,
    OPTIONS,
    PUT,
    PATCH,
    CONNECT,
    TRACE,
}

impl AllowedMethods {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DELETE => "DELETE",
            Self::GET => "GET",
            Self::HEAD => "HEAD",
            Self::MERGE => "MERGE",
            Self::POST => "POST",
            Self::OPTIONS => "OPTIONS",
            Self::PUT => "PUT",
            Self::PATCH => "PATCH",
            Self::CONNECT => "CONNECT",
            Self::TRACE => "TRACE",
        }
    }
}

impl From<AllowedMethods> for serde_json::Value {
    fn from(v: AllowedMethods) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AllowedMethods {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AllowedMethods {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "DELETE" => Ok(Self::DELETE),
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "MERGE" => Ok(Self::MERGE),
            "POST" => Ok(Self::POST),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(serde::de::Error::unknown_variant(&s, &["DELETE", "GET", "HEAD", "MERGE", "POST", "OPTIONS", "PUT", "PATCH", "CONNECT", "TRACE"])),
        }
    }
}
