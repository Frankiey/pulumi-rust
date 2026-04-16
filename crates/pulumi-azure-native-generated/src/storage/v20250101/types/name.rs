/// Name of the policy. The valid value is AccessTimeTracking. This field is currently read only
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Name {
    AccessTimeTracking,
}

impl Name {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AccessTimeTracking => "AccessTimeTracking",
        }
    }
}

impl From<Name> for serde_json::Value {
    fn from(v: Name) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for Name {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Name {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AccessTimeTracking" => Ok(Self::AccessTimeTracking),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AccessTimeTracking"])),
        }
    }
}
