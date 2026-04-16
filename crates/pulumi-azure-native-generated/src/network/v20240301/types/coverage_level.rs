/// Test coverage for the endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageLevel {
    Default,
    Low,
    BelowAverage,
    Average,
    AboveAverage,
    Full,
}

impl CoverageLevel {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Low => "Low",
            Self::BelowAverage => "BelowAverage",
            Self::Average => "Average",
            Self::AboveAverage => "AboveAverage",
            Self::Full => "Full",
        }
    }
}

impl From<CoverageLevel> for serde_json::Value {
    fn from(v: CoverageLevel) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for CoverageLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for CoverageLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Default" => Ok(Self::Default),
            "Low" => Ok(Self::Low),
            "BelowAverage" => Ok(Self::BelowAverage),
            "Average" => Ok(Self::Average),
            "AboveAverage" => Ok(Self::AboveAverage),
            "Full" => Ok(Self::Full),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Default", "Low", "BelowAverage", "Average", "AboveAverage", "Full"])),
        }
    }
}
