/// The Pfs Groups used in IKE Phase 2 for new child SA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PfsGroup {
    None,
    PFS1,
    PFS2,
    PFS2048,
    ECP256,
    ECP384,
    PFS24,
    PFS14,
    PFSMM,
}

impl PfsGroup {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::PFS1 => "PFS1",
            Self::PFS2 => "PFS2",
            Self::PFS2048 => "PFS2048",
            Self::ECP256 => "ECP256",
            Self::ECP384 => "ECP384",
            Self::PFS24 => "PFS24",
            Self::PFS14 => "PFS14",
            Self::PFSMM => "PFSMM",
        }
    }
}

impl From<PfsGroup> for serde_json::Value {
    fn from(v: PfsGroup) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PfsGroup {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PfsGroup {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "PFS1" => Ok(Self::PFS1),
            "PFS2" => Ok(Self::PFS2),
            "PFS2048" => Ok(Self::PFS2048),
            "ECP256" => Ok(Self::ECP256),
            "ECP384" => Ok(Self::ECP384),
            "PFS24" => Ok(Self::PFS24),
            "PFS14" => Ok(Self::PFS14),
            "PFSMM" => Ok(Self::PFSMM),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "PFS1", "PFS2", "PFS2048", "ECP256", "ECP384", "PFS24", "PFS14", "PFSMM"])),
        }
    }
}
