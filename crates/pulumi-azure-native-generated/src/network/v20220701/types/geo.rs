/// The Geo for CIDR advertising. Should be an Geo code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Geo {
    GLOBAL,
    AFRI,
    APAC,
    EURO,
    LATAM,
    NAM,
    ME,
    OCEANIA,
    AQ,
}

impl Geo {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GLOBAL => "GLOBAL",
            Self::AFRI => "AFRI",
            Self::APAC => "APAC",
            Self::EURO => "EURO",
            Self::LATAM => "LATAM",
            Self::NAM => "NAM",
            Self::ME => "ME",
            Self::OCEANIA => "OCEANIA",
            Self::AQ => "AQ",
        }
    }
}

impl From<Geo> for serde_json::Value {
    fn from(v: Geo) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for Geo {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Geo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "GLOBAL" => Ok(Self::GLOBAL),
            "AFRI" => Ok(Self::AFRI),
            "APAC" => Ok(Self::APAC),
            "EURO" => Ok(Self::EURO),
            "LATAM" => Ok(Self::LATAM),
            "NAM" => Ok(Self::NAM),
            "ME" => Ok(Self::ME),
            "OCEANIA" => Ok(Self::OCEANIA),
            "AQ" => Ok(Self::AQ),
            _ => Err(serde::de::Error::unknown_variant(&s, &["GLOBAL", "AFRI", "APAC", "EURO", "LATAM", "NAM", "ME", "OCEANIA", "AQ"])),
        }
    }
}
