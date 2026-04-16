/// Determines how new connections are handled by the load balancer when all backend instances are probed down.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeNoHealthyBackendsBehavior {
    /// No new flows will be sent to the backend pool.
    AllProbedDown,
    /// When all backend instances are probed down, incoming packets will be sent to all instances.
    AllProbedUp,
}

impl ProbeNoHealthyBackendsBehavior {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AllProbedDown => "AllProbedDown",
            Self::AllProbedUp => "AllProbedUp",
        }
    }
}

impl From<ProbeNoHealthyBackendsBehavior> for serde_json::Value {
    fn from(v: ProbeNoHealthyBackendsBehavior) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ProbeNoHealthyBackendsBehavior {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ProbeNoHealthyBackendsBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AllProbedDown" => Ok(Self::AllProbedDown),
            "AllProbedUp" => Ok(Self::AllProbedUp),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AllProbedDown", "AllProbedUp"])),
        }
    }
}
