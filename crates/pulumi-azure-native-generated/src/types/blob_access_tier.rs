/// The access tier of a storage blob.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobAccessTier {
    /// Optimized for storing data that is accessed frequently.
    Hot,
    /// Optimized for storing data that is infrequently accessed and stored for at least 30 days.
    Cool,
    /// Optimized for storing data that is rarely accessed and stored for at least 180 days with flexible latency requirements, on the order of hours.
    Archive,
}

impl BlobAccessTier {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hot => "Hot",
            Self::Cool => "Cool",
            Self::Archive => "Archive",
        }
    }
}

impl From<BlobAccessTier> for serde_json::Value {
    fn from(v: BlobAccessTier) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for BlobAccessTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for BlobAccessTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Hot" => Ok(Self::Hot),
            "Cool" => Ok(Self::Cool),
            "Archive" => Ok(Self::Archive),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Hot", "Cool", "Archive"])),
        }
    }
}
