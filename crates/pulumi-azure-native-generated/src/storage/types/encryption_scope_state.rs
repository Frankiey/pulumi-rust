/// The state of the encryption scope. Possible values (case-insensitive):  Enabled, Disabled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncryptionScopeState {
    Enabled,
    Disabled,
}

impl EncryptionScopeState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}

impl From<EncryptionScopeState> for serde_json::Value {
    fn from(v: EncryptionScopeState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for EncryptionScopeState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for EncryptionScopeState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Enabled", "Disabled"])),
        }
    }
}
