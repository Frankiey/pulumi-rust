/// The variable on which we evaluate the exception condition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExceptionEntryMatchVariable {
    RequestURI,
    RemoteAddr,
    RequestHeader,
}

impl ExceptionEntryMatchVariable {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RequestURI => "RequestURI",
            Self::RemoteAddr => "RemoteAddr",
            Self::RequestHeader => "RequestHeader",
        }
    }
}

impl From<ExceptionEntryMatchVariable> for serde_json::Value {
    fn from(v: ExceptionEntryMatchVariable) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExceptionEntryMatchVariable {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExceptionEntryMatchVariable {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "RequestURI" => Ok(Self::RequestURI),
            "RemoteAddr" => Ok(Self::RemoteAddr),
            "RequestHeader" => Ok(Self::RequestHeader),
            _ => Err(serde::de::Error::unknown_variant(&s, &["RequestURI", "RemoteAddr", "RequestHeader"])),
        }
    }
}
