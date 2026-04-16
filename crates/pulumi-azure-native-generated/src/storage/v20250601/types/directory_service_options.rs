/// Indicates the directory service used. Note that this enum may be extended in the future.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectoryServiceOptions {
    None,
    AADDS,
    AD,
    AADKERB,
}

impl DirectoryServiceOptions {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::AADDS => "AADDS",
            Self::AD => "AD",
            Self::AADKERB => "AADKERB",
        }
    }
}

impl From<DirectoryServiceOptions> for serde_json::Value {
    fn from(v: DirectoryServiceOptions) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DirectoryServiceOptions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryServiceOptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "AADDS" => Ok(Self::AADDS),
            "AD" => Ok(Self::AD),
            "AADKERB" => Ok(Self::AADKERB),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "AADDS", "AD", "AADKERB"])),
        }
    }
}
