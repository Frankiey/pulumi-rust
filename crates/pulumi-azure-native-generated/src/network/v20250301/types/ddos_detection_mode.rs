/// The detection mode for the DDoS detection rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DdosDetectionMode {
    TrafficThreshold,
}

impl DdosDetectionMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TrafficThreshold => "TrafficThreshold",
        }
    }
}

impl From<DdosDetectionMode> for serde_json::Value {
    fn from(v: DdosDetectionMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DdosDetectionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DdosDetectionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "TrafficThreshold" => Ok(Self::TrafficThreshold),
            _ => Err(serde::de::Error::unknown_variant(&s, &["TrafficThreshold"])),
        }
    }
}
