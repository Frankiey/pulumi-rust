/// The signed services accessible with the service SAS. Possible values include: Blob (b), Container (c), File (f), Share (s).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignedResource {
    B,
    C,
    F,
    S,
}

impl SignedResource {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::B => "b",
            Self::C => "c",
            Self::F => "f",
            Self::S => "s",
        }
    }
}

impl From<SignedResource> for serde_json::Value {
    fn from(v: SignedResource) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SignedResource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SignedResource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "f" => Ok(Self::F),
            "s" => Ok(Self::S),
            _ => Err(serde::de::Error::unknown_variant(&s, &["b", "c", "f", "s"])),
        }
    }
}
