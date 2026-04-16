/// The Vpn Policy member attribute type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VpnPolicyMemberAttributeType {
    CertificateGroupId,
    AADGroupId,
    RadiusAzureGroupId,
}

impl VpnPolicyMemberAttributeType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CertificateGroupId => "CertificateGroupId",
            Self::AADGroupId => "AADGroupId",
            Self::RadiusAzureGroupId => "RadiusAzureGroupId",
        }
    }
}

impl From<VpnPolicyMemberAttributeType> for serde_json::Value {
    fn from(v: VpnPolicyMemberAttributeType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for VpnPolicyMemberAttributeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for VpnPolicyMemberAttributeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "CertificateGroupId" => Ok(Self::CertificateGroupId),
            "AADGroupId" => Ok(Self::AADGroupId),
            "RadiusAzureGroupId" => Ok(Self::RadiusAzureGroupId),
            _ => Err(serde::de::Error::unknown_variant(&s, &["CertificateGroupId", "AADGroupId", "RadiusAzureGroupId"])),
        }
    }
}
