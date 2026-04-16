/// Access tier for specific share. GpV2 account can choose between TransactionOptimized (default), Hot, and Cool. FileStorage account can choose Premium.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShareAccessTier {
    TransactionOptimized,
    Hot,
    Cool,
    Premium,
}

impl ShareAccessTier {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TransactionOptimized => "TransactionOptimized",
            Self::Hot => "Hot",
            Self::Cool => "Cool",
            Self::Premium => "Premium",
        }
    }
}

impl From<ShareAccessTier> for serde_json::Value {
    fn from(v: ShareAccessTier) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ShareAccessTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ShareAccessTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "TransactionOptimized" => Ok(Self::TransactionOptimized),
            "Hot" => Ok(Self::Hot),
            "Cool" => Ok(Self::Cool),
            "Premium" => Ok(Self::Premium),
            _ => Err(serde::de::Error::unknown_variant(&s, &["TransactionOptimized", "Hot", "Cool", "Premium"])),
        }
    }
}
