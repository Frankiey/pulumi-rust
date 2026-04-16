/// Type of custom IP prefix. Should be Singular, Parent, or Child.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomIpPrefixType {
    Singular,
    Parent,
    Child,
}

impl CustomIpPrefixType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Singular => "Singular",
            Self::Parent => "Parent",
            Self::Child => "Child",
        }
    }
}

impl From<CustomIpPrefixType> for serde_json::Value {
    fn from(v: CustomIpPrefixType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for CustomIpPrefixType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for CustomIpPrefixType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Singular" => Ok(Self::Singular),
            "Parent" => Ok(Self::Parent),
            "Child" => Ok(Self::Child),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Singular", "Parent", "Child"])),
        }
    }
}
