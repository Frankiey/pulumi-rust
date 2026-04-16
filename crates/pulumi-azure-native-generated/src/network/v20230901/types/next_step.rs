/// Next step after rule is evaluated. Current supported behaviors are 'Continue'(to next rule) and 'Terminate'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextStep {
    Unknown,
    Continue,
    Terminate,
}

impl NextStep {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::Continue => "Continue",
            Self::Terminate => "Terminate",
        }
    }
}

impl From<NextStep> for serde_json::Value {
    fn from(v: NextStep) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for NextStep {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for NextStep {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Unknown" => Ok(Self::Unknown),
            "Continue" => Ok(Self::Continue),
            "Terminate" => Ok(Self::Terminate),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Unknown", "Continue", "Terminate"])),
        }
    }
}
