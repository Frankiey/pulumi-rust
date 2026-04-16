/// The access mode of the private link service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessMode {
    /// Allows unrestricted access to the private link service.
    Default,
    /// Limits access to subscriptions which are inside visibility list only.
    Restricted,
}

impl AccessMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Restricted => "Restricted",
        }
    }
}

impl From<AccessMode> for serde_json::Value {
    fn from(v: AccessMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AccessMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AccessMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Default" => Ok(Self::Default),
            "Restricted" => Ok(Self::Restricted),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Default", "Restricted"])),
        }
    }
}
