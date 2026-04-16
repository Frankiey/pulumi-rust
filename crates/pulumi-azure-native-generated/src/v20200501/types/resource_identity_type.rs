/// The type of identity used for the resource. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the virtual machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceIdentityType {
    SystemAssigned,
    UserAssigned,
    SystemAssignedUserAssigned,
    None,
}

impl ResourceIdentityType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SystemAssigned => "SystemAssigned",
            Self::UserAssigned => "UserAssigned",
            Self::SystemAssignedUserAssigned => "SystemAssigned, UserAssigned",
            Self::None => "None",
        }
    }
}

impl From<ResourceIdentityType> for serde_json::Value {
    fn from(v: ResourceIdentityType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ResourceIdentityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ResourceIdentityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "SystemAssigned" => Ok(Self::SystemAssigned),
            "UserAssigned" => Ok(Self::UserAssigned),
            "SystemAssigned, UserAssigned" => Ok(Self::SystemAssignedUserAssigned),
            "None" => Ok(Self::None),
            _ => Err(serde::de::Error::unknown_variant(&s, &["SystemAssigned", "UserAssigned", "SystemAssigned, UserAssigned", "None"])),
        }
    }
}
