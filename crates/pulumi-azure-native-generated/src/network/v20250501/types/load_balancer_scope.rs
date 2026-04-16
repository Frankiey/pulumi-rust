/// Indicates the scope of the load balancer: external (Public) or internal (Private).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadBalancerScope {
    Public,
    Private,
}

impl LoadBalancerScope {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "Public",
            Self::Private => "Private",
        }
    }
}

impl From<LoadBalancerScope> for serde_json::Value {
    fn from(v: LoadBalancerScope) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for LoadBalancerScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for LoadBalancerScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Public" => Ok(Self::Public),
            "Private" => Ok(Self::Private),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Public", "Private"])),
        }
    }
}
