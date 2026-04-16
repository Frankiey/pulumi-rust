/// Match Variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebApplicationFirewallMatchVariable {
    RemoteAddr,
    RequestMethod,
    QueryString,
    PostArgs,
    RequestUri,
    RequestHeaders,
    RequestBody,
    RequestCookies,
}

impl WebApplicationFirewallMatchVariable {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RemoteAddr => "RemoteAddr",
            Self::RequestMethod => "RequestMethod",
            Self::QueryString => "QueryString",
            Self::PostArgs => "PostArgs",
            Self::RequestUri => "RequestUri",
            Self::RequestHeaders => "RequestHeaders",
            Self::RequestBody => "RequestBody",
            Self::RequestCookies => "RequestCookies",
        }
    }
}

impl From<WebApplicationFirewallMatchVariable> for serde_json::Value {
    fn from(v: WebApplicationFirewallMatchVariable) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for WebApplicationFirewallMatchVariable {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for WebApplicationFirewallMatchVariable {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "RemoteAddr" => Ok(Self::RemoteAddr),
            "RequestMethod" => Ok(Self::RequestMethod),
            "QueryString" => Ok(Self::QueryString),
            "PostArgs" => Ok(Self::PostArgs),
            "RequestUri" => Ok(Self::RequestUri),
            "RequestHeaders" => Ok(Self::RequestHeaders),
            "RequestBody" => Ok(Self::RequestBody),
            "RequestCookies" => Ok(Self::RequestCookies),
            _ => Err(serde::de::Error::unknown_variant(&s, &["RemoteAddr", "RequestMethod", "QueryString", "PostArgs", "RequestUri", "RequestHeaders", "RequestBody", "RequestCookies"])),
        }
    }
}
