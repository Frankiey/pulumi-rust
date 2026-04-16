/// Specify what happens to the public IP address when the VM using it is deleted
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeleteOptions {
    Delete,
    Detach,
}

impl DeleteOptions {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Delete => "Delete",
            Self::Detach => "Detach",
        }
    }
}

impl From<DeleteOptions> for serde_json::Value {
    fn from(v: DeleteOptions) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DeleteOptions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DeleteOptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Delete" => Ok(Self::Delete),
            "Detach" => Ok(Self::Detach),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Delete", "Detach"])),
        }
    }
}
