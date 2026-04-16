/// The type of a storage blob to be created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobType {
    /// Block blobs store text and binary data. Block blobs are made up of blocks of data that can be managed individually.
    Block,
    /// Append blobs are made up of blocks like block blobs, but are optimized for append operations.
    Append,
}

impl BlobType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Block => "Block",
            Self::Append => "Append",
        }
    }
}

impl From<BlobType> for serde_json::Value {
    fn from(v: BlobType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for BlobType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for BlobType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Block" => Ok(Self::Block),
            "Append" => Ok(Self::Append),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Block", "Append"])),
        }
    }
}
