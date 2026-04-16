/// The identity type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    SystemAssignedUserAssigned,
}

impl IdentityType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::SystemAssigned => "SystemAssigned",
            Self::UserAssigned => "UserAssigned",
            Self::SystemAssignedUserAssigned => "SystemAssigned,UserAssigned",
        }
    }
}

impl From<IdentityType> for serde_json::Value {
    fn from(v: IdentityType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for IdentityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for IdentityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "SystemAssigned" => Ok(Self::SystemAssigned),
            "UserAssigned" => Ok(Self::UserAssigned),
            "SystemAssigned,UserAssigned" => Ok(Self::SystemAssignedUserAssigned),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "SystemAssigned", "UserAssigned", "SystemAssigned,UserAssigned"])),
        }
    }
}
