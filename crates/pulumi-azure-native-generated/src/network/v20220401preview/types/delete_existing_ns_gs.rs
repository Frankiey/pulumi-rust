/// Flag if need to delete existing network security groups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeleteExistingNSGs {
    False,
    True,
}

impl DeleteExistingNSGs {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::False => "False",
            Self::True => "True",
        }
    }
}

impl From<DeleteExistingNSGs> for serde_json::Value {
    fn from(v: DeleteExistingNSGs) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DeleteExistingNSGs {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DeleteExistingNSGs {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "False" => Ok(Self::False),
            "True" => Ok(Self::True),
            _ => Err(serde::de::Error::unknown_variant(&s, &["False", "True"])),
        }
    }
}
