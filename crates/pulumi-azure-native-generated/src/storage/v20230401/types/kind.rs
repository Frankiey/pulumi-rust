/// Required. Indicates the type of storage account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Storage,
    StorageV2,
    BlobStorage,
    FileStorage,
    BlockBlobStorage,
}

impl Kind {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Storage => "Storage",
            Self::StorageV2 => "StorageV2",
            Self::BlobStorage => "BlobStorage",
            Self::FileStorage => "FileStorage",
            Self::BlockBlobStorage => "BlockBlobStorage",
        }
    }
}

impl From<Kind> for serde_json::Value {
    fn from(v: Kind) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for Kind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Kind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Storage" => Ok(Self::Storage),
            "StorageV2" => Ok(Self::StorageV2),
            "BlobStorage" => Ok(Self::BlobStorage),
            "FileStorage" => Ok(Self::FileStorage),
            "BlockBlobStorage" => Ok(Self::BlockBlobStorage),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Storage", "StorageV2", "BlobStorage", "FileStorage", "BlockBlobStorage"])),
        }
    }
}
