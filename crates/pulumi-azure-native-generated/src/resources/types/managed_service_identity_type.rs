/// Type of the managed identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagedServiceIdentityType {
    UserAssigned,
}

impl ManagedServiceIdentityType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UserAssigned => "UserAssigned",
        }
    }
}

impl From<ManagedServiceIdentityType> for serde_json::Value {
    fn from(v: ManagedServiceIdentityType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ManagedServiceIdentityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "UserAssigned" => Ok(Self::UserAssigned),
            _ => Err(serde::de::Error::unknown_variant(&s, &["UserAssigned"])),
        }
    }
}
