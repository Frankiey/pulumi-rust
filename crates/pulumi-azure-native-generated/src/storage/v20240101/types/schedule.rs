/// This is a required field. This field is used to schedule an inventory formation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Schedule {
    Daily,
    Weekly,
}

impl Schedule {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Daily => "Daily",
            Self::Weekly => "Weekly",
        }
    }
}

impl From<Schedule> for serde_json::Value {
    fn from(v: Schedule) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for Schedule {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Schedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Daily" => Ok(Self::Daily),
            "Weekly" => Ok(Self::Weekly),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Daily", "Weekly"])),
        }
    }
}
