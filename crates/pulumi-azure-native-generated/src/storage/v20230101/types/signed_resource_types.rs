/// The signed resource types that are accessible with the account SAS. Service (s): Access to service-level APIs; Container (c): Access to container-level APIs; Object (o): Access to object-level APIs for blobs, queue messages, table entities, and files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignedResourceTypes {
    S,
    C,
    O,
}

impl SignedResourceTypes {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::S => "s",
            Self::C => "c",
            Self::O => "o",
        }
    }
}

impl From<SignedResourceTypes> for serde_json::Value {
    fn from(v: SignedResourceTypes) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SignedResourceTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SignedResourceTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "s" => Ok(Self::S),
            "c" => Ok(Self::C),
            "o" => Ok(Self::O),
            _ => Err(serde::de::Error::unknown_variant(&s, &["s", "c", "o"])),
        }
    }
}
