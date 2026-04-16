/// Gets the state of virtual network rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Provisioning,
    Deprovisioning,
    Succeeded,
    Failed,
    NetworkSourceDeleted,
}

impl State {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Provisioning => "Provisioning",
            Self::Deprovisioning => "Deprovisioning",
            Self::Succeeded => "Succeeded",
            Self::Failed => "Failed",
            Self::NetworkSourceDeleted => "NetworkSourceDeleted",
        }
    }
}

impl From<State> for serde_json::Value {
    fn from(v: State) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for State {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for State {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Provisioning" => Ok(Self::Provisioning),
            "Deprovisioning" => Ok(Self::Deprovisioning),
            "Succeeded" => Ok(Self::Succeeded),
            "Failed" => Ok(Self::Failed),
            "NetworkSourceDeleted" => Ok(Self::NetworkSourceDeleted),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Provisioning", "Deprovisioning", "Succeeded", "Failed", "NetworkSourceDeleted"])),
        }
    }
}
