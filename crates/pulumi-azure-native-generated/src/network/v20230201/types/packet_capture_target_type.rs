/// Target type of the resource provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketCaptureTargetType {
    AzureVM,
    AzureVMSS,
}

impl PacketCaptureTargetType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AzureVM => "AzureVM",
            Self::AzureVMSS => "AzureVMSS",
        }
    }
}

impl From<PacketCaptureTargetType> for serde_json::Value {
    fn from(v: PacketCaptureTargetType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PacketCaptureTargetType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PacketCaptureTargetType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "AzureVM" => Ok(Self::AzureVM),
            "AzureVMSS" => Ok(Self::AzureVMSS),
            _ => Err(serde::de::Error::unknown_variant(&s, &["AzureVM", "AzureVMSS"])),
        }
    }
}
