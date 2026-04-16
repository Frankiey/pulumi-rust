/// Type of Actions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebApplicationFirewallAction {
    Allow,
    Block,
    Log,
}

impl WebApplicationFirewallAction {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Allow => "Allow",
            Self::Block => "Block",
            Self::Log => "Log",
        }
    }
}

impl From<WebApplicationFirewallAction> for serde_json::Value {
    fn from(v: WebApplicationFirewallAction) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for WebApplicationFirewallAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for WebApplicationFirewallAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Allow" => Ok(Self::Allow),
            "Block" => Ok(Self::Block),
            "Log" => Ok(Self::Log),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Allow", "Block", "Log"])),
        }
    }
}
