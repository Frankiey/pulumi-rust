/// Mac security cipher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressRouteLinkMacSecCipher {
    GcmAes128,
    GcmAes256,
}

impl ExpressRouteLinkMacSecCipher {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GcmAes128 => "gcm-aes-128",
            Self::GcmAes256 => "gcm-aes-256",
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
            "gcm-aes-128" => Ok(Self::GcmAes128),
            "gcm-aes-256" => Ok(Self::GcmAes256),
            _ => Err(serde::de::Error::unknown_variant(&s, &["gcm-aes-128", "gcm-aes-256"])),
        }
    }
}
