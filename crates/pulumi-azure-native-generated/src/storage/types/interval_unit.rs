/// Run interval unit of task execution. This is a required field when ExecutionTrigger.properties.type is 'OnSchedule'; this property should not be present when ExecutionTrigger.properties.type is 'RunOnce'
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalUnit {
    Days,
}

impl IntervalUnit {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Days => "Days",
        }
    }
}

impl From<IntervalUnit> for serde_json::Value {
    fn from(v: IntervalUnit) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for IntervalUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for IntervalUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Days" => Ok(Self::Days),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Days"])),
        }
    }
}
