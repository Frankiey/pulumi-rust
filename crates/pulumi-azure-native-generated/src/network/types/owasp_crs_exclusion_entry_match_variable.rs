/// The variable to be excluded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwaspCrsExclusionEntryMatchVariable {
    RequestHeaderNames,
    RequestCookieNames,
    RequestArgNames,
    RequestHeaderKeys,
    RequestHeaderValues,
    RequestCookieKeys,
    RequestCookieValues,
    RequestArgKeys,
    RequestArgValues,
}

impl OwaspCrsExclusionEntryMatchVariable {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RequestHeaderNames => "RequestHeaderNames",
            Self::RequestCookieNames => "RequestCookieNames",
            Self::RequestArgNames => "RequestArgNames",
            Self::RequestHeaderKeys => "RequestHeaderKeys",
            Self::RequestHeaderValues => "RequestHeaderValues",
            Self::RequestCookieKeys => "RequestCookieKeys",
            Self::RequestCookieValues => "RequestCookieValues",
            Self::RequestArgKeys => "RequestArgKeys",
            Self::RequestArgValues => "RequestArgValues",
        }
    }
}

impl From<OwaspCrsExclusionEntryMatchVariable> for serde_json::Value {
    fn from(v: OwaspCrsExclusionEntryMatchVariable) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for OwaspCrsExclusionEntryMatchVariable {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for OwaspCrsExclusionEntryMatchVariable {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "RequestHeaderNames" => Ok(Self::RequestHeaderNames),
            "RequestCookieNames" => Ok(Self::RequestCookieNames),
            "RequestArgNames" => Ok(Self::RequestArgNames),
            "RequestHeaderKeys" => Ok(Self::RequestHeaderKeys),
            "RequestHeaderValues" => Ok(Self::RequestHeaderValues),
            "RequestCookieKeys" => Ok(Self::RequestCookieKeys),
            "RequestCookieValues" => Ok(Self::RequestCookieValues),
            "RequestArgKeys" => Ok(Self::RequestArgKeys),
            "RequestArgValues" => Ok(Self::RequestArgValues),
            _ => Err(serde::de::Error::unknown_variant(&s, &["RequestHeaderNames", "RequestCookieNames", "RequestArgNames", "RequestHeaderKeys", "RequestHeaderValues", "RequestCookieKeys", "RequestCookieValues", "RequestArgKeys", "RequestArgValues"])),
        }
    }
}
