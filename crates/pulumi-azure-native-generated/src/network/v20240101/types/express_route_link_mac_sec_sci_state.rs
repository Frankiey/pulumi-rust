/// Sci mode enabled/disabled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressRouteLinkMacSecSciState {
    Disabled,
    Enabled,
}

impl ExpressRouteLinkMacSecSciState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }
}

impl From<ExpressRouteLinkMacSecSciState> for serde_json::Value {
    fn from(v: ExpressRouteLinkMacSecSciState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExpressRouteLinkMacSecSciState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpressRouteLinkMacSecSciState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Disabled" => Ok(Self::Disabled),
            "Enabled" => Ok(Self::Enabled),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Disabled", "Enabled"])),
        }
    }
}
