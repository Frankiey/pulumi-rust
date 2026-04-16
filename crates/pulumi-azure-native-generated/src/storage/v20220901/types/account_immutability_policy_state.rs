/// The ImmutabilityPolicy state defines the mode of the policy. Disabled state disables the policy, Unlocked state allows increase and decrease of immutability retention time and also allows toggling allowProtectedAppendWrites property, Locked state only allows the increase of the immutability retention time. A policy can only be created in a Disabled or Unlocked state and can be toggled between the two states. Only a policy in an Unlocked state can transition to a Locked state which cannot be reverted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountImmutabilityPolicyState {
    Unlocked,
    Locked,
    Disabled,
}

impl AccountImmutabilityPolicyState {
    /// The wire value sent to the Pulumi engine.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unlocked => "Unlocked",
            Self::Locked => "Locked",
            Self::Disabled => "Disabled",
        }
    }
}

impl From<AccountImmutabilityPolicyState> for serde_json::Value {
    fn from(v: AccountImmutabilityPolicyState) -> Self {
        serde_json::Value::String(v.as_str().to_string())
    }
}

impl serde::Serialize for AccountImmutabilityPolicyState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AccountImmutabilityPolicyState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Unlocked" => Ok(Self::Unlocked),
            "Locked" => Ok(Self::Locked),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Unlocked", "Locked", "Disabled"])),
        }
    }
}
