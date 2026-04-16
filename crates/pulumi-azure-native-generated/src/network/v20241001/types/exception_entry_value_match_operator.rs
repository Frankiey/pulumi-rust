/// Operates on the allowed values for the matchVariable
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExceptionEntryValueMatchOperator {
    Equals,
    Contains,
    StartsWith,
    EndsWith,
    IPMatch,
}

impl ExceptionEntryValueMatchOperator {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equals => "Equals",
            Self::Contains => "Contains",
            Self::StartsWith => "StartsWith",
            Self::EndsWith => "EndsWith",
            Self::IPMatch => "IPMatch",
        }
    }
}

impl From<ExceptionEntryValueMatchOperator> for serde_json::Value {
    fn from(v: ExceptionEntryValueMatchOperator) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExceptionEntryValueMatchOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExceptionEntryValueMatchOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Equals" => Ok(Self::Equals),
            "Contains" => Ok(Self::Contains),
            "StartsWith" => Ok(Self::StartsWith),
            "EndsWith" => Ok(Self::EndsWith),
            "IPMatch" => Ok(Self::IPMatch),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Equals", "Contains", "StartsWith", "EndsWith", "IPMatch"])),
        }
    }
}
