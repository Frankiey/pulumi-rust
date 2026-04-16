/// The variable to be scrubbed from the logs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrubbingRuleEntryMatchVariable {
    RequestHeaderNames,
    RequestCookieNames,
    RequestArgNames,
    RequestPostArgNames,
    RequestJSONArgNames,
    RequestIPAddress,
}

impl ScrubbingRuleEntryMatchVariable {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RequestHeaderNames => "RequestHeaderNames",
            Self::RequestCookieNames => "RequestCookieNames",
            Self::RequestArgNames => "RequestArgNames",
            Self::RequestPostArgNames => "RequestPostArgNames",
            Self::RequestJSONArgNames => "RequestJSONArgNames",
            Self::RequestIPAddress => "RequestIPAddress",
        }
    }
}

impl From<ScrubbingRuleEntryMatchVariable> for serde_json::Value {
    fn from(v: ScrubbingRuleEntryMatchVariable) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ScrubbingRuleEntryMatchVariable {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ScrubbingRuleEntryMatchVariable {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "RequestHeaderNames" => Ok(Self::RequestHeaderNames),
            "RequestCookieNames" => Ok(Self::RequestCookieNames),
            "RequestArgNames" => Ok(Self::RequestArgNames),
            "RequestPostArgNames" => Ok(Self::RequestPostArgNames),
            "RequestJSONArgNames" => Ok(Self::RequestJSONArgNames),
            "RequestIPAddress" => Ok(Self::RequestIPAddress),
            _ => Err(serde::de::Error::unknown_variant(&s, &["RequestHeaderNames", "RequestCookieNames", "RequestArgNames", "RequestPostArgNames", "RequestJSONArgNames", "RequestIPAddress"])),
        }
    }
}
