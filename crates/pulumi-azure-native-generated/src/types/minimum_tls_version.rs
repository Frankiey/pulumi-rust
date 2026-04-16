/// Set the minimum TLS version to be permitted on requests to storage. The default interpretation is TLS 1.0 for this property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MinimumTlsVersion {
    TLS10,
    TLS11,
    TLS12,
    TLS13,
}

impl MinimumTlsVersion {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TLS10 => "TLS1_0",
            Self::TLS11 => "TLS1_1",
            Self::TLS12 => "TLS1_2",
            Self::TLS13 => "TLS1_3",
        }
    }
}

impl From<MinimumTlsVersion> for serde_json::Value {
    fn from(v: MinimumTlsVersion) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for MinimumTlsVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for MinimumTlsVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "TLS1_0" => Ok(Self::TLS10),
            "TLS1_1" => Ok(Self::TLS11),
            "TLS1_2" => Ok(Self::TLS12),
            "TLS1_3" => Ok(Self::TLS13),
            _ => Err(serde::de::Error::unknown_variant(&s, &["TLS1_0", "TLS1_1", "TLS1_2", "TLS1_3"])),
        }
    }
}
