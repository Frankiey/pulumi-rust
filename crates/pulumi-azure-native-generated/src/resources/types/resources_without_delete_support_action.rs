/// Some resources do not support deletion.  This flag will denote how the stack should handle those resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourcesWithoutDeleteSupportAction {
    /// Detach the specified resources from the deployment stack and continue.
    Detach,
    /// Fail the deployment stack if resources cannot be deleted.
    Fail,
}

impl ResourcesWithoutDeleteSupportAction {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Detach => "detach",
            Self::Fail => "fail",
        }
    }
}

impl From<ResourcesWithoutDeleteSupportAction> for serde_json::Value {
    fn from(v: ResourcesWithoutDeleteSupportAction) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ResourcesWithoutDeleteSupportAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ResourcesWithoutDeleteSupportAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "detach" => Ok(Self::Detach),
            "fail" => Ok(Self::Fail),
            _ => Err(serde::de::Error::unknown_variant(&s, &["detach", "fail"])),
        }
    }
}
