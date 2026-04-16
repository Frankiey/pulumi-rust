/// RNM supported protocol types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolType {
    DoNotUse,
    Icmp,
    Tcp,
    Udp,
    Gre,
    Esp,
    Ah,
    Vxlan,
    All,
}

impl ProtocolType {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DoNotUse => "DoNotUse",
            Self::Icmp => "Icmp",
            Self::Tcp => "Tcp",
            Self::Udp => "Udp",
            Self::Gre => "Gre",
            Self::Esp => "Esp",
            Self::Ah => "Ah",
            Self::Vxlan => "Vxlan",
            Self::All => "All",
        }
    }
}

impl From<ProtocolType> for serde_json::Value {
    fn from(v: ProtocolType) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for ProtocolType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ProtocolType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "DoNotUse" => Ok(Self::DoNotUse),
            "Icmp" => Ok(Self::Icmp),
            "Tcp" => Ok(Self::Tcp),
            "Udp" => Ok(Self::Udp),
            "Gre" => Ok(Self::Gre),
            "Esp" => Ok(Self::Esp),
            "Ah" => Ok(Self::Ah),
            "Vxlan" => Ok(Self::Vxlan),
            "All" => Ok(Self::All),
            _ => Err(serde::de::Error::unknown_variant(&s, &["DoNotUse", "Icmp", "Tcp", "Udp", "Gre", "Esp", "Ah", "Vxlan", "All"])),
        }
    }
}
