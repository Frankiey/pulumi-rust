/// The endpoint type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndpointType {
    AzureVM,
    AzureVNet,
    AzureSubnet,
    ExternalAddress,
    MMAWorkspaceMachine,
    MMAWorkspaceNetwork,
    AzureArcVM,
    AzureVMSS,
}

impl EndpointType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AzureVM => "AzureVM",
            Self::AzureVNet => "AzureVNet",
            Self::AzureSubnet => "AzureSubnet",
            Self::ExternalAddress => "ExternalAddress",
            Self::MMAWorkspaceMachine => "MMAWorkspaceMachine",
            Self::MMAWorkspaceNetwork => "MMAWorkspaceNetwork",
            Self::AzureArcVM => "AzureArcVM",
            Self::AzureVMSS => "AzureVMSS",
        }
    }
}

impl From<EndpointType> for serde_json::Value {
    fn from(v: EndpointType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for EndpointType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for EndpointType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AzureVM" => Ok(Self::AzureVM),
            "AzureVNet" => Ok(Self::AzureVNet),
            "AzureSubnet" => Ok(Self::AzureSubnet),
            "ExternalAddress" => Ok(Self::ExternalAddress),
            "MMAWorkspaceMachine" => Ok(Self::MMAWorkspaceMachine),
            "MMAWorkspaceNetwork" => Ok(Self::MMAWorkspaceNetwork),
            "AzureArcVM" => Ok(Self::AzureArcVM),
            "AzureVMSS" => Ok(Self::AzureVMSS),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AzureVM", "AzureVNet", "AzureSubnet", "ExternalAddress", "MMAWorkspaceMachine", "MMAWorkspaceNetwork", "AzureArcVM", "AzureVMSS"])),
        }
    }
}
