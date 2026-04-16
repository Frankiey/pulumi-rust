/// The signed services accessible with the account SAS. Possible values include: Blob (b), Queue (q), Table (t), File (f).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Services {
    B,
    Q,
    T,
    F,
}

impl Services {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::B => "b",
            Self::Q => "q",
            Self::T => "t",
            Self::F => "f",
        }
    }
}

impl From<Services> for serde_json::Value {
    fn from(v: Services) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for Services {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Services {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "b" => Ok(Self::B),
            "q" => Ok(Self::Q),
            "t" => Ok(Self::T),
            "f" => Ok(Self::F),
            _ => Err(serde::de::Error::unknown_variant(&s, &["b", "q", "t", "f"])),
        }
    }
}
