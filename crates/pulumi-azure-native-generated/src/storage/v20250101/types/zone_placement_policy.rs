/// The availability zone pinning policy for the storage account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZonePlacementPolicy {
    Any,
    None,
}

impl ZonePlacementPolicy {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Any => "Any",
            Self::None => "None",
        }
    }
}

impl From<ZonePlacementPolicy> for serde_json::Value {
    fn from(v: ZonePlacementPolicy) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ZonePlacementPolicy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ZonePlacementPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Any" => Ok(Self::Any),
            "None" => Ok(Self::None),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Any", "None"])),
        }
    }
}
