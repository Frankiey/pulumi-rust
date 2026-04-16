/// HTTP redirection type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationGatewayRedirectType {
    Permanent,
    Found,
    SeeOther,
    Temporary,
}

impl ApplicationGatewayRedirectType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Permanent => "Permanent",
            Self::Found => "Found",
            Self::SeeOther => "SeeOther",
            Self::Temporary => "Temporary",
        }
    }
}

impl From<ApplicationGatewayRedirectType> for serde_json::Value {
    fn from(v: ApplicationGatewayRedirectType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ApplicationGatewayRedirectType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ApplicationGatewayRedirectType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Permanent" => Ok(Self::Permanent),
            "Found" => Ok(Self::Found),
            "SeeOther" => Ok(Self::SeeOther),
            "Temporary" => Ok(Self::Temporary),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Permanent", "Found", "SeeOther", "Temporary"])),
        }
    }
}
