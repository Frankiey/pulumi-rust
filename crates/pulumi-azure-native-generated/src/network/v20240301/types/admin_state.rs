/// Property to indicate if the Express Route Gateway serves traffic when there are multiple Express Route Gateways in the vnet
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminState {
    Enabled,
    Disabled,
}

impl AdminState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}

impl From<AdminState> for serde_json::Value {
    fn from(v: AdminState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AdminState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AdminState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Enabled", "Disabled"])),
        }
    }
}
