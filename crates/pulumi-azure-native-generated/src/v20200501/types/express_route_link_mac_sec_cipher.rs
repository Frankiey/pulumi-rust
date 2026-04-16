/// Mac security cipher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressRouteLinkMacSecCipher {
    GcmAes256,
    GcmAes128,
}

impl ExpressRouteLinkMacSecCipher {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GcmAes256 => "GcmAes256",
            Self::GcmAes128 => "GcmAes128",
        }
    }
}

impl From<ExpressRouteLinkMacSecCipher> for serde_json::Value {
    fn from(v: ExpressRouteLinkMacSecCipher) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExpressRouteLinkMacSecCipher {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpressRouteLinkMacSecCipher {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "GcmAes256" => Ok(Self::GcmAes256),
            "GcmAes128" => Ok(Self::GcmAes128),
            _ => Err(serde::de::Error::unknown_variant(&s, &["GcmAes256", "GcmAes128"])),
        }
    }
}
