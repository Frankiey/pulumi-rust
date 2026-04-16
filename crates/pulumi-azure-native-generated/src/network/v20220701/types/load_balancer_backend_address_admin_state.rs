/// A list of administrative states which once set can override health probe so that Load Balancer will always forward new connections to backend, or deny new connections and reset existing connections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadBalancerBackendAddressAdminState {
    None,
    Up,
    Down,
    Drain,
}

impl LoadBalancerBackendAddressAdminState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Drain => "Drain",
        }
    }
}

impl From<LoadBalancerBackendAddressAdminState> for serde_json::Value {
    fn from(v: LoadBalancerBackendAddressAdminState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for LoadBalancerBackendAddressAdminState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for LoadBalancerBackendAddressAdminState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "Up" => Ok(Self::Up),
            "Down" => Ok(Self::Down),
            "Drain" => Ok(Self::Drain),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "Up", "Down", "Drain"])),
        }
    }
}
