/// Backend address synchronous mode for the backend pool
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncMode {
    Automatic,
    Manual,
}

impl SyncMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Automatic => "Automatic",
            Self::Manual => "Manual",
        }
    }
}

impl From<SyncMode> for serde_json::Value {
    fn from(v: SyncMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SyncMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SyncMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Automatic" => Ok(Self::Automatic),
            "Manual" => Ok(Self::Manual),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Automatic", "Manual"])),
        }
    }
}
