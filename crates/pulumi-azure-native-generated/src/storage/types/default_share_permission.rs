/// Default share permission for users using Kerberos authentication if RBAC role is not assigned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefaultSharePermission {
    None,
    StorageFileDataSmbShareReader,
    StorageFileDataSmbShareContributor,
    StorageFileDataSmbShareElevatedContributor,
}

impl DefaultSharePermission {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::StorageFileDataSmbShareReader => "StorageFileDataSmbShareReader",
            Self::StorageFileDataSmbShareContributor => "StorageFileDataSmbShareContributor",
            Self::StorageFileDataSmbShareElevatedContributor => "StorageFileDataSmbShareElevatedContributor",
        }
    }
}

impl From<DefaultSharePermission> for serde_json::Value {
    fn from(v: DefaultSharePermission) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DefaultSharePermission {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DefaultSharePermission {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "StorageFileDataSmbShareReader" => Ok(Self::StorageFileDataSmbShareReader),
            "StorageFileDataSmbShareContributor" => Ok(Self::StorageFileDataSmbShareContributor),
            "StorageFileDataSmbShareElevatedContributor" => Ok(Self::StorageFileDataSmbShareElevatedContributor),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "StorageFileDataSmbShareReader", "StorageFileDataSmbShareContributor", "StorageFileDataSmbShareElevatedContributor"])),
        }
    }
}
