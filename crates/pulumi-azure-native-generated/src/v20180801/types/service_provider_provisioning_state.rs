/// The ServiceProviderProvisioningState state of the resource. Possible values are 'NotProvisioned', 'Provisioning', 'Provisioned', and 'Deprovisioning'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceProviderProvisioningState {
    NotProvisioned,
    Provisioning,
    Provisioned,
    Deprovisioning,
}

impl ServiceProviderProvisioningState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotProvisioned => "NotProvisioned",
            Self::Provisioning => "Provisioning",
            Self::Provisioned => "Provisioned",
            Self::Deprovisioning => "Deprovisioning",
        }
    }
}

impl From<ServiceProviderProvisioningState> for serde_json::Value {
    fn from(v: ServiceProviderProvisioningState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ServiceProviderProvisioningState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ServiceProviderProvisioningState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "NotProvisioned" => Ok(Self::NotProvisioned),
            "Provisioning" => Ok(Self::Provisioning),
            "Provisioned" => Ok(Self::Provisioned),
            "Deprovisioning" => Ok(Self::Deprovisioning),
            _ => Err(serde::de::Error::unknown_variant(&s, &["NotProvisioned", "Provisioning", "Provisioned", "Deprovisioning"])),
        }
    }
}
