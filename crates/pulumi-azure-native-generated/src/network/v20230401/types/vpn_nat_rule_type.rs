/// The type of NAT rule for VPN NAT.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VpnNatRuleType {
    Static,
    Dynamic,
}

impl VpnNatRuleType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Static => "Static",
            Self::Dynamic => "Dynamic",
        }
    }
}

impl From<VpnNatRuleType> for serde_json::Value {
    fn from(v: VpnNatRuleType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VpnNatRuleType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VpnNatRuleType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Static" => Ok(Self::Static),
            "Dynamic" => Ok(Self::Dynamic),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Static", "Dynamic"])),
        }
    }
}
