/// Private Endpoint VNet Policies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrivateEndpointVNetPolicies {
    Disabled,
    Basic,
}

impl PrivateEndpointVNetPolicies {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Disabled => "Disabled",
            Self::Basic => "Basic",
        }
    }
}

impl From<PrivateEndpointVNetPolicies> for serde_json::Value {
    fn from(v: PrivateEndpointVNetPolicies) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PrivateEndpointVNetPolicies {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PrivateEndpointVNetPolicies {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Disabled" => Ok(Self::Disabled),
            "Basic" => Ok(Self::Basic),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Disabled", "Basic"])),
        }
    }
}
