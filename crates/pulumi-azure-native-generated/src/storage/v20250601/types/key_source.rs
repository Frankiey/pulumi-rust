/// The encryption keySource (provider). Possible values (case-insensitive):  Microsoft.Storage, Microsoft.Keyvault
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeySource {
    MicrosoftStorage,
    MicrosoftKeyvault,
}

impl KeySource {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MicrosoftStorage => "Microsoft.Storage",
            Self::MicrosoftKeyvault => "Microsoft.Keyvault",
        }
    }
}

impl From<KeySource> for serde_json::Value {
    fn from(v: KeySource) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for KeySource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for KeySource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Microsoft.Storage" => Ok(Self::MicrosoftStorage),
            "Microsoft.Keyvault" => Ok(Self::MicrosoftKeyvault),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Microsoft.Storage", "Microsoft.Keyvault"])),
        }
    }
}
