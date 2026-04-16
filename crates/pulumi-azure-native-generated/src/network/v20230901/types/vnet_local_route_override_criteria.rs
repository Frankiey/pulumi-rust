/// Parameter determining whether NVA in spoke vnet is bypassed for traffic with destination in spoke.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VnetLocalRouteOverrideCriteria {
    Contains,
    Equal,
}

impl VnetLocalRouteOverrideCriteria {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Contains => "Contains",
            Self::Equal => "Equal",
        }
    }
}

impl From<VnetLocalRouteOverrideCriteria> for serde_json::Value {
    fn from(v: VnetLocalRouteOverrideCriteria) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VnetLocalRouteOverrideCriteria {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VnetLocalRouteOverrideCriteria {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Contains" => Ok(Self::Contains),
            "Equal" => Ok(Self::Equal),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Contains", "Equal"])),
        }
    }
}
