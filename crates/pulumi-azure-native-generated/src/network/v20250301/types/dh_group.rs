/// The DH Group used in IKE Phase 1 for initial SA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DhGroup {
    None,
    DHGroup1,
    DHGroup2,
    DHGroup14,
    DHGroup2048,
    ECP256,
    ECP384,
    DHGroup24,
}

impl DhGroup {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::DHGroup1 => "DHGroup1",
            Self::DHGroup2 => "DHGroup2",
            Self::DHGroup14 => "DHGroup14",
            Self::DHGroup2048 => "DHGroup2048",
            Self::ECP256 => "ECP256",
            Self::ECP384 => "ECP384",
            Self::DHGroup24 => "DHGroup24",
        }
    }
}

impl From<DhGroup> for serde_json::Value {
    fn from(v: DhGroup) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for DhGroup {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for DhGroup {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "None" => Ok(Self::None),
            "DHGroup1" => Ok(Self::DHGroup1),
            "DHGroup2" => Ok(Self::DHGroup2),
            "DHGroup14" => Ok(Self::DHGroup14),
            "DHGroup2048" => Ok(Self::DHGroup2048),
            "ECP256" => Ok(Self::ECP256),
            "ECP384" => Ok(Self::ECP384),
            "DHGroup24" => Ok(Self::DHGroup24),
            _ => Err(serde::de::Error::unknown_variant(&s, &["None", "DHGroup1", "DHGroup2", "DHGroup14", "DHGroup2048", "ECP256", "ECP384", "DHGroup24"])),
        }
    }
}
