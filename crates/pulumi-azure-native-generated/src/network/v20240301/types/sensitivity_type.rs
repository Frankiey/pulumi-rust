/// Describes the override sensitivity to be applied when rule matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SensitivityType {
    None,
    Low,
    Medium,
    High,
}

impl SensitivityType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }
}

impl From<SensitivityType> for serde_json::Value {
    fn from(v: SensitivityType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SensitivityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SensitivityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "Low" => Ok(Self::Low),
            "Medium" => Ok(Self::Medium),
            "High" => Ok(Self::High),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "Low", "Medium", "High"])),
        }
    }
}
