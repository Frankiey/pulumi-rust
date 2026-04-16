/// Specifies the Active Directory account type for Azure Storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountType {
    User,
    Computer,
}

impl AccountType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "User",
            Self::Computer => "Computer",
        }
    }
}

impl From<AccountType> for serde_json::Value {
    fn from(v: AccountType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AccountType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "User" => Ok(Self::User),
            "Computer" => Ok(Self::Computer),
            _ => Err(serde::de::Error::unknown_variant(&s, &["User", "Computer"])),
        }
    }
}
