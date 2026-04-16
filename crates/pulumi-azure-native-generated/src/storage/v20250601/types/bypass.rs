/// Specifies whether traffic is bypassed for Logging/Metrics/AzureServices. Possible values are any combination of Logging|Metrics|AzureServices (For example, "Logging, Metrics"), or None to bypass none of those traffics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bypass {
    None,
    Logging,
    Metrics,
    AzureServices,
}

impl Bypass {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Logging => "Logging",
            Self::Metrics => "Metrics",
            Self::AzureServices => "AzureServices",
        }
    }
}

impl From<Bypass> for serde_json::Value {
    fn from(v: Bypass) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for Bypass {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Bypass {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "Logging" => Ok(Self::Logging),
            "Metrics" => Ok(Self::Metrics),
            "AzureServices" => Ok(Self::AzureServices),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "Logging", "Metrics", "AzureServices"])),
        }
    }
}
