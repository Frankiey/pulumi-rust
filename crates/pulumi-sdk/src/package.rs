/// Parameterization for a package registration.
///
/// Used when registering parameterized providers via `register_package`.
#[derive(Debug, Clone)]
pub struct Parameterization {
    /// The parameterized package name.
    pub name: String,
    /// The parameterized package version.
    pub version: String,
    /// The parameter value (opaque bytes).
    pub value: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameterization_fields() {
        let p = Parameterization {
            name: "my-pkg".to_string(),
            version: "1.0.0".to_string(),
            value: vec![1, 2, 3],
        };
        assert_eq!(p.name, "my-pkg");
        assert_eq!(p.version, "1.0.0");
        assert_eq!(p.value, vec![1, 2, 3]);
    }
}
