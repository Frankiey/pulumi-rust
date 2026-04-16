/// This is a required field. This field specifies the scope of the inventory created either at the blob or container level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Blob,
    Container,
}

impl ObjectType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blob => "Blob",
            Self::Container => "Container",
        }
    }
}

impl From<ObjectType> for serde_json::Value {
    fn from(v: ObjectType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ObjectType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ObjectType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Blob" => Ok(Self::Blob),
            "Container" => Ok(Self::Container),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Blob", "Container"])),
        }
    }
}
