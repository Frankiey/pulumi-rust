/// Security Type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityType {
    AdminPolicy,
    UserPolicy,
}

impl SecurityType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdminPolicy => "AdminPolicy",
            Self::UserPolicy => "UserPolicy",
        }
    }
}

impl From<SecurityType> for serde_json::Value {
    fn from(v: SecurityType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SecurityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SecurityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AdminPolicy" => Ok(Self::AdminPolicy),
            "UserPolicy" => Ok(Self::UserPolicy),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AdminPolicy", "UserPolicy"])),
        }
    }
}
