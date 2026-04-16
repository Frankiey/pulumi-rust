/// The load distribution policy for this rule. Possible values are 'Default', 'SourceIP', and 'SourceIPProtocol'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadDistribution {
    Default,
    SourceIP,
    SourceIPProtocol,
}

impl LoadDistribution {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::SourceIP => "SourceIP",
            Self::SourceIPProtocol => "SourceIPProtocol",
        }
    }
}

impl From<LoadDistribution> for serde_json::Value {
    fn from(v: LoadDistribution) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for LoadDistribution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for LoadDistribution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Default" => Ok(Self::Default),
            "SourceIP" => Ok(Self::SourceIP),
            "SourceIPProtocol" => Ok(Self::SourceIPProtocol),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Default", "SourceIP", "SourceIPProtocol"])),
        }
    }
}
