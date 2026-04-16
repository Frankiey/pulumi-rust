/// The authorization use status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationUseStatus {
    Available,
    InUse,
}

impl AuthorizationUseStatus {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::InUse => "InUse",
        }
    }
}

impl From<AuthorizationUseStatus> for serde_json::Value {
    fn from(v: AuthorizationUseStatus) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AuthorizationUseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AuthorizationUseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Available" => Ok(Self::Available),
            "InUse" => Ok(Self::InUse),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Available", "InUse"])),
        }
    }
}
