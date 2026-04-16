/// The provider for the encryption scope. Possible values (case-insensitive):  Microsoft.Storage, Microsoft.KeyVault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncryptionScopeSource {
    MicrosoftStorage,
    MicrosoftKeyVault,
}

impl EncryptionScopeSource {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MicrosoftStorage => "Microsoft.Storage",
            Self::MicrosoftKeyVault => "Microsoft.KeyVault",
        }
    }
}

impl From<EncryptionScopeSource> for serde_json::Value {
    fn from(v: EncryptionScopeSource) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for EncryptionScopeSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for EncryptionScopeSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Microsoft.Storage" => Ok(Self::MicrosoftStorage),
            "Microsoft.KeyVault" => Ok(Self::MicrosoftKeyVault),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Microsoft.Storage", "Microsoft.KeyVault"])),
        }
    }
}
