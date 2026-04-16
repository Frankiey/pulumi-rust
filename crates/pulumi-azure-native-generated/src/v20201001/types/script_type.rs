/// Type of the script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptType {
    AzurePowerShell,
    AzureCLI,
}

impl ScriptType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AzurePowerShell => "AzurePowerShell",
            Self::AzureCLI => "AzureCLI",
        }
    }
}

impl From<ScriptType> for serde_json::Value {
    fn from(v: ScriptType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ScriptType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ScriptType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AzurePowerShell" => Ok(Self::AzurePowerShell),
            "AzureCLI" => Ok(Self::AzureCLI),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AzurePowerShell", "AzureCLI"])),
        }
    }
}
