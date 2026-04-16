/// The commissioned state of the Custom IP Prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommissionedState {
    Provisioning,
    Provisioned,
    Commissioning,
    CommissionedNoInternetAdvertise,
    Commissioned,
    Decommissioning,
    Deprovisioning,
    Deprovisioned,
}

impl CommissionedState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Provisioning => "Provisioning",
            Self::Provisioned => "Provisioned",
            Self::Commissioning => "Commissioning",
            Self::CommissionedNoInternetAdvertise => "CommissionedNoInternetAdvertise",
            Self::Commissioned => "Commissioned",
            Self::Decommissioning => "Decommissioning",
            Self::Deprovisioning => "Deprovisioning",
            Self::Deprovisioned => "Deprovisioned",
        }
    }
}

impl From<CommissionedState> for serde_json::Value {
    fn from(v: CommissionedState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for CommissionedState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for CommissionedState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Provisioning" => Ok(Self::Provisioning),
            "Provisioned" => Ok(Self::Provisioned),
            "Commissioning" => Ok(Self::Commissioning),
            "CommissionedNoInternetAdvertise" => Ok(Self::CommissionedNoInternetAdvertise),
            "Commissioned" => Ok(Self::Commissioned),
            "Decommissioning" => Ok(Self::Decommissioning),
            "Deprovisioning" => Ok(Self::Deprovisioning),
            "Deprovisioned" => Ok(Self::Deprovisioned),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Provisioning", "Provisioned", "Commissioning", "CommissionedNoInternetAdvertise", "Commissioned", "Decommissioning", "Deprovisioning", "Deprovisioned"])),
        }
    }
}
