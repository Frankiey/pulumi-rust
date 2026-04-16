/// The protocol for the outbound rule in load balancer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadBalancerOutboundRuleProtocol {
    Tcp,
    Udp,
    All,
}

impl LoadBalancerOutboundRuleProtocol {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "Tcp",
            Self::Udp => "Udp",
            Self::All => "All",
        }
    }
}

impl From<LoadBalancerOutboundRuleProtocol> for serde_json::Value {
    fn from(v: LoadBalancerOutboundRuleProtocol) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for LoadBalancerOutboundRuleProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for LoadBalancerOutboundRuleProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Tcp" => Ok(Self::Tcp),
            "Udp" => Ok(Self::Udp),
            "All" => Ok(Self::All),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Tcp", "Udp", "All"])),
        }
    }
}
