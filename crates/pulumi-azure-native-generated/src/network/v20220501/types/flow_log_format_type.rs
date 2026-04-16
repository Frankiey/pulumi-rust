/// The file type of flow log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowLogFormatType {
    JSON,
}

impl FlowLogFormatType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::JSON => "JSON",
        }
    }
}

impl From<FlowLogFormatType> for serde_json::Value {
    fn from(v: FlowLogFormatType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for FlowLogFormatType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FlowLogFormatType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "JSON" => Ok(Self::JSON),
            _ => Err(serde::de::Error::unknown_variant(&s, &["JSON"])),
        }
    }
}
