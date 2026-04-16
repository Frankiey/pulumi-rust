/// The SKU name. Required for account creation; optional for update. Note that in older versions, SKU name was called accountType.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkuName {
    StandardLRS,
    StandardGRS,
    StandardRAGRS,
    StandardZRS,
    PremiumLRS,
    PremiumZRS,
    StandardGZRS,
    StandardRAGZRS,
    StandardV2LRS,
    StandardV2GRS,
    StandardV2ZRS,
    StandardV2GZRS,
    PremiumV2LRS,
    PremiumV2ZRS,
}

impl SkuName {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StandardLRS => "Standard_LRS",
            Self::StandardGRS => "Standard_GRS",
            Self::StandardRAGRS => "Standard_RAGRS",
            Self::StandardZRS => "Standard_ZRS",
            Self::PremiumLRS => "Premium_LRS",
            Self::PremiumZRS => "Premium_ZRS",
            Self::StandardGZRS => "Standard_GZRS",
            Self::StandardRAGZRS => "Standard_RAGZRS",
            Self::StandardV2LRS => "StandardV2_LRS",
            Self::StandardV2GRS => "StandardV2_GRS",
            Self::StandardV2ZRS => "StandardV2_ZRS",
            Self::StandardV2GZRS => "StandardV2_GZRS",
            Self::PremiumV2LRS => "PremiumV2_LRS",
            Self::PremiumV2ZRS => "PremiumV2_ZRS",
        }
    }
}

impl From<SkuName> for serde_json::Value {
    fn from(v: SkuName) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for SkuName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SkuName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Standard_LRS" => Ok(Self::StandardLRS),
            "Standard_GRS" => Ok(Self::StandardGRS),
            "Standard_RAGRS" => Ok(Self::StandardRAGRS),
            "Standard_ZRS" => Ok(Self::StandardZRS),
            "Premium_LRS" => Ok(Self::PremiumLRS),
            "Premium_ZRS" => Ok(Self::PremiumZRS),
            "Standard_GZRS" => Ok(Self::StandardGZRS),
            "Standard_RAGZRS" => Ok(Self::StandardRAGZRS),
            "StandardV2_LRS" => Ok(Self::StandardV2LRS),
            "StandardV2_GRS" => Ok(Self::StandardV2GRS),
            "StandardV2_ZRS" => Ok(Self::StandardV2ZRS),
            "StandardV2_GZRS" => Ok(Self::StandardV2GZRS),
            "PremiumV2_LRS" => Ok(Self::PremiumV2LRS),
            "PremiumV2_ZRS" => Ok(Self::PremiumV2ZRS),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Standard_LRS", "Standard_GRS", "Standard_RAGRS", "Standard_ZRS", "Premium_LRS", "Premium_ZRS", "Standard_GZRS", "Standard_RAGZRS", "StandardV2_LRS", "StandardV2_GRS", "StandardV2_ZRS", "StandardV2_GZRS", "PremiumV2_LRS", "PremiumV2_ZRS"])),
        }
    }
}
