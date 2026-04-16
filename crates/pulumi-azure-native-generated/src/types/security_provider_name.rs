/// The security provider name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityProviderName {
    ZScaler,
    IBoss,
    Checkpoint,
}

impl SecurityProviderName {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ZScaler => "ZScaler",
            Self::IBoss => "IBoss",
            Self::Checkpoint => "Checkpoint",
        }
    }
}

impl From<SecurityProviderName> for serde_json::Value {
    fn from(v: SecurityProviderName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SecurityProviderName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SecurityProviderName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ZScaler" => Ok(Self::ZScaler),
            "IBoss" => Ok(Self::IBoss),
            "Checkpoint" => Ok(Self::Checkpoint),
            _ => Err(serde::de::Error::unknown_variant(&s, &["ZScaler", "IBoss", "Checkpoint"])),
        }
    }
}
