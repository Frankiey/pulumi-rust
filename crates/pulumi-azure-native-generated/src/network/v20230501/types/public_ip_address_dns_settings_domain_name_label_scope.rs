/// The domain name label scope. If a domain name label and a domain name label scope are specified, an A DNS record is created for the public IP in the Microsoft Azure DNS system with a hashed value includes in FQDN.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicIpAddressDnsSettingsDomainNameLabelScope {
    TenantReuse,
    SubscriptionReuse,
    ResourceGroupReuse,
    NoReuse,
}

impl PublicIpAddressDnsSettingsDomainNameLabelScope {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TenantReuse => "TenantReuse",
            Self::SubscriptionReuse => "SubscriptionReuse",
            Self::ResourceGroupReuse => "ResourceGroupReuse",
            Self::NoReuse => "NoReuse",
        }
    }
}

impl From<PublicIpAddressDnsSettingsDomainNameLabelScope> for serde_json::Value {
    fn from(v: PublicIpAddressDnsSettingsDomainNameLabelScope) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for PublicIpAddressDnsSettingsDomainNameLabelScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PublicIpAddressDnsSettingsDomainNameLabelScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "TenantReuse" => Ok(Self::TenantReuse),
            "SubscriptionReuse" => Ok(Self::SubscriptionReuse),
            "ResourceGroupReuse" => Ok(Self::ResourceGroupReuse),
            "NoReuse" => Ok(Self::NoReuse),
            _ => Err(serde::de::Error::unknown_variant(&s, &["TenantReuse", "SubscriptionReuse", "ResourceGroupReuse", "NoReuse"])),
        }
    }
}
