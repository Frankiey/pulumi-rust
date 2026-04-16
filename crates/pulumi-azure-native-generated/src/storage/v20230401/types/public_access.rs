/// Specifies whether data in the container may be accessed publicly and the level of access.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicAccess {
    Container,
    Blob,
    None,
}

impl PublicAccess {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Container => "Container",
            Self::Blob => "Blob",
            Self::None => "None",
        }
    }
}

impl From<PublicAccess> for serde_json::Value {
    fn from(v: PublicAccess) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PublicAccess {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PublicAccess {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Container" => Ok(Self::Container),
            "Blob" => Ok(Self::Blob),
            "None" => Ok(Self::None),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Container", "Blob", "None"])),
        }
    }
}
