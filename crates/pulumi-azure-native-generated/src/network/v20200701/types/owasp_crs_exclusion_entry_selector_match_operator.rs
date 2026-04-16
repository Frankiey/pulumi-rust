/// When matchVariable is a collection, operate on the selector to specify which elements in the collection this exclusion applies to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwaspCrsExclusionEntrySelectorMatchOperator {
    Equals,
    Contains,
    StartsWith,
    EndsWith,
    EqualsAny,
}

impl OwaspCrsExclusionEntrySelectorMatchOperator {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equals => "Equals",
            Self::Contains => "Contains",
            Self::StartsWith => "StartsWith",
            Self::EndsWith => "EndsWith",
            Self::EqualsAny => "EqualsAny",
        }
    }
}

impl From<OwaspCrsExclusionEntrySelectorMatchOperator> for serde_json::Value {
    fn from(v: OwaspCrsExclusionEntrySelectorMatchOperator) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for OwaspCrsExclusionEntrySelectorMatchOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for OwaspCrsExclusionEntrySelectorMatchOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Equals" => Ok(Self::Equals),
            "Contains" => Ok(Self::Contains),
            "StartsWith" => Ok(Self::StartsWith),
            "EndsWith" => Ok(Self::EndsWith),
            "EqualsAny" => Ok(Self::EqualsAny),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Equals", "Contains", "StartsWith", "EndsWith", "EqualsAny"])),
        }
    }
}
