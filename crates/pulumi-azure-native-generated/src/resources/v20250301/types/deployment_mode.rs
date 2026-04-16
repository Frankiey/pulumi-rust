/// The mode that is used to deploy resources. This value can be either Incremental or Complete. In Incremental mode, resources are deployed without deleting existing resources that are not included in the template. In Complete mode, resources are deployed and existing resources in the resource group that are not included in the template are deleted. Be careful when using Complete mode as you may unintentionally delete resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeploymentMode {
    Incremental,
    Complete,
}

impl DeploymentMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Incremental => "Incremental",
            Self::Complete => "Complete",
        }
    }
}

impl From<DeploymentMode> for serde_json::Value {
    fn from(v: DeploymentMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DeploymentMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DeploymentMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Incremental" => Ok(Self::Incremental),
            "Complete" => Ok(Self::Complete),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Incremental", "Complete"])),
        }
    }
}
