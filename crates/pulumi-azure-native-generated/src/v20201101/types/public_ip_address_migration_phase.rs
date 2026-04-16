/// Migration phase of Public IP Address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicIPAddressMigrationPhase {
    None,
    Prepare,
    Commit,
    Abort,
    Committed,
}

impl PublicIPAddressMigrationPhase {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Prepare => "Prepare",
            Self::Commit => "Commit",
            Self::Abort => "Abort",
            Self::Committed => "Committed",
        }
    }
}

impl From<PublicIPAddressMigrationPhase> for serde_json::Value {
    fn from(v: PublicIPAddressMigrationPhase) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PublicIPAddressMigrationPhase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PublicIPAddressMigrationPhase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "Prepare" => Ok(Self::Prepare),
            "Commit" => Ok(Self::Commit),
            "Abort" => Ok(Self::Abort),
            "Committed" => Ok(Self::Committed),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "Prepare", "Commit", "Abort", "Committed"])),
        }
    }
}
