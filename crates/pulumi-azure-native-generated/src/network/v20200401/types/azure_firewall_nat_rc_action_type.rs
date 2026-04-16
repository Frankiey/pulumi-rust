/// The type of action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AzureFirewallNatRCActionType {
    Snat,
    Dnat,
}

impl AzureFirewallNatRCActionType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Snat => "Snat",
            Self::Dnat => "Dnat",
        }
    }
}

impl From<AzureFirewallNatRCActionType> for serde_json::Value {
    fn from(v: AzureFirewallNatRCActionType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AzureFirewallNatRCActionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AzureFirewallNatRCActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Snat" => Ok(Self::Snat),
            "Dnat" => Ok(Self::Dnat),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Snat", "Dnat"])),
        }
    }
}
