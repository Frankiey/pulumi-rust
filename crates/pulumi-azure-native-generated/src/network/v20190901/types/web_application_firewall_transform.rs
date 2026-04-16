/// Describes what transforms applied before matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebApplicationFirewallTransform {
    Lowercase,
    Trim,
    UrlDecode,
    UrlEncode,
    RemoveNulls,
    HtmlEntityDecode,
}

impl WebApplicationFirewallTransform {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lowercase => "Lowercase",
            Self::Trim => "Trim",
            Self::UrlDecode => "UrlDecode",
            Self::UrlEncode => "UrlEncode",
            Self::RemoveNulls => "RemoveNulls",
            Self::HtmlEntityDecode => "HtmlEntityDecode",
        }
    }
}

impl From<WebApplicationFirewallTransform> for serde_json::Value {
    fn from(v: WebApplicationFirewallTransform) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for WebApplicationFirewallTransform {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for WebApplicationFirewallTransform {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Lowercase" => Ok(Self::Lowercase),
            "Trim" => Ok(Self::Trim),
            "UrlDecode" => Ok(Self::UrlDecode),
            "UrlEncode" => Ok(Self::UrlEncode),
            "RemoveNulls" => Ok(Self::RemoveNulls),
            "HtmlEntityDecode" => Ok(Self::HtmlEntityDecode),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Lowercase", "Trim", "UrlDecode", "UrlEncode", "RemoveNulls", "HtmlEntityDecode"])),
        }
    }
}
