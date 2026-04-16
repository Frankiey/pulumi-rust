/// The deployment on error behavior type. Possible values are LastSuccessful and SpecificDeployment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnErrorDeploymentType {
    LastSuccessful,
    SpecificDeployment,
}

impl OnErrorDeploymentType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LastSuccessful => "LastSuccessful",
            Self::SpecificDeployment => "SpecificDeployment",
        }
    }
}

impl From<OnErrorDeploymentType> for serde_json::Value {
    fn from(v: OnErrorDeploymentType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for OnErrorDeploymentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for OnErrorDeploymentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "LastSuccessful" => Ok(Self::LastSuccessful),
            "SpecificDeployment" => Ok(Self::SpecificDeployment),
            _ => Err(serde::de::Error::unknown_variant(&s, &["LastSuccessful", "SpecificDeployment"])),
        }
    }
}
