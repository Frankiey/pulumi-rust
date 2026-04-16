/// Describes if it is in detection mode or prevention mode at policy level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebApplicationFirewallMode {
    Prevention,
    Detection,
}

impl WebApplicationFirewallMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Prevention => "Prevention",
            Self::Detection => "Detection",
        }
    }
}

impl From<WebApplicationFirewallMode> for serde_json::Value {
    fn from(v: WebApplicationFirewallMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for WebApplicationFirewallMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for WebApplicationFirewallMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Prevention" => Ok(Self::Prevention),
            "Detection" => Ok(Self::Detection),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Prevention", "Detection"])),
        }
    }
}
