/// The property is for NFS share only. The default is NoRootSquash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootSquashType {
    NoRootSquash,
    RootSquash,
    AllSquash,
}

impl RootSquashType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoRootSquash => "NoRootSquash",
            Self::RootSquash => "RootSquash",
            Self::AllSquash => "AllSquash",
        }
    }
}

impl From<RootSquashType> for serde_json::Value {
    fn from(v: RootSquashType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for RootSquashType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for RootSquashType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "NoRootSquash" => Ok(Self::NoRootSquash),
            "RootSquash" => Ok(Self::RootSquash),
            "AllSquash" => Ok(Self::AllSquash),
            _ => Err(serde::de::Error::unknown_variant(&s, &["NoRootSquash", "RootSquash", "AllSquash"])),
        }
    }
}
