/// Access mode on the association.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssociationAccessMode {
    Learning,
    Enforced,
    Audit,
}

impl AssociationAccessMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Learning => "Learning",
            Self::Enforced => "Enforced",
            Self::Audit => "Audit",
        }
    }
}

impl From<AssociationAccessMode> for serde_json::Value {
    fn from(v: AssociationAccessMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AssociationAccessMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AssociationAccessMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Learning" => Ok(Self::Learning),
            "Enforced" => Ok(Self::Enforced),
            "Audit" => Ok(Self::Audit),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Learning", "Enforced", "Audit"])),
        }
    }
}
