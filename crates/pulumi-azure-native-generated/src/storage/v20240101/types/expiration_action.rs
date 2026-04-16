/// The SAS Expiration Action defines the action to be performed when sasPolicy.sasExpirationPeriod is violated. The 'Log' action can be used for audit purposes and the 'Block' action can be used to block and deny the usage of SAS tokens that do not adhere to the sas policy expiration period.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpirationAction {
    Log,
    Block,
}

impl ExpirationAction {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Log => "Log",
            Self::Block => "Block",
        }
    }
}

impl From<ExpirationAction> for serde_json::Value {
    fn from(v: ExpirationAction) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExpirationAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpirationAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Log" => Ok(Self::Log),
            "Block" => Ok(Self::Block),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Log", "Block"])),
        }
    }
}
