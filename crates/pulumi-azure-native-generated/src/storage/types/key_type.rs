/// Encryption key type to be used for the encryption service. 'Account' key type implies that an account-scoped encryption key will be used. 'Service' key type implies that a default service key is used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyType {
    Service,
    Account,
}

impl KeyType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Service => "Service",
            Self::Account => "Account",
        }
    }
}

impl From<KeyType> for serde_json::Value {
    fn from(v: KeyType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for KeyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for KeyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Service" => Ok(Self::Service),
            "Account" => Ok(Self::Account),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Service", "Account"])),
        }
    }
}
