/// The validation level of the deployment stack
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationLevel {
    /// Static analysis of the template is performed.
    Template,
    /// Static analysis of the template is performed and resource declarations are sent to resource providers for semantic validation. Validates that the caller has RBAC write permissions on each resource.
    Provider,
    /// Static analysis of the template is performed and resource declarations are sent to resource providers for semantic validation. Skips validating that the caller has RBAC write permissions on each resource.
    ProviderNoRbac,
}

impl ValidationLevel {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Template => "Template",
            Self::Provider => "Provider",
            Self::ProviderNoRbac => "ProviderNoRbac",
        }
    }
}

impl From<ValidationLevel> for serde_json::Value {
    fn from(v: ValidationLevel) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ValidationLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ValidationLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Template" => Ok(Self::Template),
            "Provider" => Ok(Self::Provider),
            "ProviderNoRbac" => Ok(Self::ProviderNoRbac),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Template", "Provider", "ProviderNoRbac"])),
        }
    }
}
