/// The HTTP method to use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HTTPConfigurationMethod {
    Get,
    Post,
}

impl HTTPConfigurationMethod {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "Get",
            Self::Post => "Post",
        }
    }
}

impl From<HTTPConfigurationMethod> for serde_json::Value {
    fn from(v: HTTPConfigurationMethod) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for HTTPConfigurationMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for HTTPConfigurationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Get" => Ok(Self::Get),
            "Post" => Ok(Self::Post),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Get", "Post"])),
        }
    }
}
