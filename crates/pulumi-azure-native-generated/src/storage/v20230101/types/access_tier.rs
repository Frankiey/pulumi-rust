/// Required for storage accounts where kind = BlobStorage. The access tier is used for billing. The 'Premium' access tier is the default value for premium block blobs storage account type and it cannot be changed for the premium block blobs storage account type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessTier {
    Hot,
    Cool,
    Premium,
}

impl AccessTier {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hot => "Hot",
            Self::Cool => "Cool",
            Self::Premium => "Premium",
        }
    }
}

impl From<AccessTier> for serde_json::Value {
    fn from(v: AccessTier) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AccessTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AccessTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Hot" => Ok(Self::Hot),
            "Cool" => Ok(Self::Cool),
            "Premium" => Ok(Self::Premium),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Hot", "Cool", "Premium"])),
        }
    }
}
