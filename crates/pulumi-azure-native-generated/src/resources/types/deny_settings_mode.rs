/// denySettings Mode that defines denied actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DenySettingsMode {
    /// Authorized users are able to read and modify the resources, but cannot delete.
    DenyDelete,
    /// Authorized users can read from a resource, but cannot modify or delete it.
    DenyWriteAndDelete,
    /// No denyAssignments have been applied.
    None,
}

impl DenySettingsMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DenyDelete => "denyDelete",
            Self::DenyWriteAndDelete => "denyWriteAndDelete",
            Self::None => "none",
        }
    }
}

impl From<DenySettingsMode> for serde_json::Value {
    fn from(v: DenySettingsMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DenySettingsMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DenySettingsMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "denyDelete" => Ok(Self::DenyDelete),
            "denyWriteAndDelete" => Ok(Self::DenyWriteAndDelete),
            "none" => Ok(Self::None),
            _ => Err(serde::de::Error::unknown_variant(&s, &["denyDelete", "denyWriteAndDelete", "none"])),
        }
    }
}
