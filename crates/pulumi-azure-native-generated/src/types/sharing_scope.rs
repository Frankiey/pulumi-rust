/// Set this property to Tenant to allow sharing subnet with other subscriptions in your AAD tenant. This property can only be set if defaultOutboundAccess is set to false, both properties can only be set if subnet is empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharingScope {
    Tenant,
    DelegatedServices,
}

impl SharingScope {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tenant => "Tenant",
            Self::DelegatedServices => "DelegatedServices",
        }
    }
}

impl From<SharingScope> for serde_json::Value {
    fn from(v: SharingScope) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SharingScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SharingScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Tenant" => Ok(Self::Tenant),
            "DelegatedServices" => Ok(Self::DelegatedServices),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Tenant", "DelegatedServices"])),
        }
    }
}
