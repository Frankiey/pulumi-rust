/// This is a required field, it specifies the format for the inventory files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    Csv,
    Parquet,
}

impl Format {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Csv => "Csv",
            Self::Parquet => "Parquet",
        }
    }
}

impl From<Format> for serde_json::Value {
    fn from(v: Format) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for Format {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Format {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Csv" => Ok(Self::Csv),
            "Parquet" => Ok(Self::Parquet),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Csv", "Parquet"])),
        }
    }
}
