/// The Source NAT direction of a VPN NAT.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VpnNatRuleMode {
    EgressSnat,
    IngressSnat,
}

impl VpnNatRuleMode {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EgressSnat => "EgressSnat",
            Self::IngressSnat => "IngressSnat",
        }
    }
}

impl From<VpnNatRuleMode> for serde_json::Value {
    fn from(v: VpnNatRuleMode) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VpnNatRuleMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VpnNatRuleMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "EgressSnat" => Ok(Self::EgressSnat),
            "IngressSnat" => Ok(Self::IngressSnat),
            _ => Err(serde::de::Error::unknown_variant(&s, &["EgressSnat", "IngressSnat"])),
        }
    }
}
