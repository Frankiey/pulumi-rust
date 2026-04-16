/// The scope to be used for evaluation of parameters, variables and functions in a nested template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionEvaluationOptionsScopeType {
    NotSpecified,
    Outer,
    Inner,
}

impl ExpressionEvaluationOptionsScopeType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotSpecified => "NotSpecified",
            Self::Outer => "Outer",
            Self::Inner => "Inner",
        }
    }
}

impl From<ExpressionEvaluationOptionsScopeType> for serde_json::Value {
    fn from(v: ExpressionEvaluationOptionsScopeType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ExpressionEvaluationOptionsScopeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpressionEvaluationOptionsScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "NotSpecified" => Ok(Self::NotSpecified),
            "Outer" => Ok(Self::Outer),
            "Inner" => Ok(Self::Inner),
            _ => Err(serde::de::Error::unknown_variant(&s, &["NotSpecified", "Outer", "Inner"])),
        }
    }
}
