/// Specifies an action for a newly unmanaged resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnmanageActionResourceMode {
    /// Delete the resources from Azure
    Delete,
    /// Keep the resources in Azure
    Detach,
}

impl UnmanageActionResourceMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Delete => "delete",
            Self::Detach => "detach",
        }
    }
}

impl From<UnmanageActionResourceMode> for serde_json::Value {
    fn from(v: UnmanageActionResourceMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for UnmanageActionResourceMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for UnmanageActionResourceMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "delete" => Ok(Self::Delete),
            "detach" => Ok(Self::Detach),
            _ => Err(serde::de::Error::unknown_variant(&s, &["delete", "detach"])),
        }
    }
}
