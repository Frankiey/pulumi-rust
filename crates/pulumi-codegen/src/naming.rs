//! Naming conventions for converting Pulumi tokens to Rust identifiers.
//!
//! Pulumi type tokens follow the format `pkg:module:TypeName`. This module
//! converts them to idiomatic Rust names: snake_case modules, PascalCase types,
//! snake_case fields.

/// Rust keywords that need escaping when used as identifiers.
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
    "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true",
    "type", "unsafe", "use", "where", "while", "yield", "abstract", "become", "box", "do",
    "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual",
];

/// Context for naming conversions within a specific package.
pub struct NamingContext {
    /// The snake_case package name (e.g., "azure_native").
    pub package_name: String,
}

impl NamingContext {
    /// Create a new naming context for the given Pulumi package name.
    ///
    /// Converts kebab-case package names to snake_case.
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: kebab_to_snake(package_name),
        }
    }

    /// Extract the module path segments from a Pulumi type token.
    ///
    /// `"pkg:module/submodule:TypeName"` → `["module", "submodule"]`
    /// `"pkg:index:TypeName"` → `[]` (index is the root module)
    pub fn token_to_module_path(&self, token: &str) -> Vec<String> {
        let parts: Vec<&str> = token.splitn(3, ':').collect();
        if parts.len() < 3 {
            return Vec::new();
        }
        let module_part = parts[1];
        let type_part = parts[2];

        let mut segments = Vec::new();

        // The module part may be "index", "index/subPath", "storage", "storage/blobService", etc.
        // "index" (and subpaths under it) maps to root level.
        let mod_segs: Vec<&str> = module_part.split('/').collect();
        if mod_segs.first() != Some(&"index") {
            for s in &mod_segs {
                segments.push(camel_to_snake(s));
            }
        }

        // If type part has a prefix (e.g., "random/terraformConfig"),
        // add all but the last segment as module path components
        let type_segments: Vec<&str> = type_part.split('/').collect();
        if type_segments.len() > 1 {
            for s in &type_segments[..type_segments.len() - 1] {
                segments.push(camel_to_snake(s));
            }
        }

        segments
    }

    /// Extract the type name from a Pulumi type token.
    ///
    /// `"pkg:module:TypeName"` → `"TypeName"`
    /// `"pkg:providers:random/terraformConfig"` → `"terraformConfig"`
    pub fn token_to_type_name(&self, token: &str) -> String {
        let parts: Vec<&str> = token.splitn(3, ':').collect();
        if parts.len() < 3 {
            return token.to_string();
        }
        let type_part = parts[2];
        // Strip any leading module prefix (e.g., "random/terraformConfig" → "terraformConfig")
        if let Some(pos) = type_part.rfind('/') {
            type_part[pos + 1..].to_string()
        } else {
            type_part.to_string()
        }
    }

    /// Convert a Pulumi property name (camelCase) to a Rust field name (snake_case).
    ///
    /// Handles keyword escaping.
    pub fn property_to_field_name(&self, property: &str) -> String {
        let snake = camel_to_snake(property);
        escape_keyword(&snake)
    }
}

/// Convert a kebab-case string to snake_case.
///
/// `"azure-native"` → `"azure_native"`
pub fn kebab_to_snake(s: &str) -> String {
    s.replace('-', "_")
}

/// Convert a camelCase or PascalCase string to snake_case.
///
/// Handles runs of uppercase letters (e.g., `"getHTTPSEnabled"` → `"get_https_enabled"`).
pub fn camel_to_snake(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(s.len() + 4);
    let chars: Vec<char> = s.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            let at_start = i == 0;
            let prev_lower = i > 0 && chars[i - 1].is_lowercase();
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();
            let prev_upper = i > 0 && chars[i - 1].is_uppercase();

            // Insert underscore before this uppercase char if:
            // - Not at the start, AND
            // - Either the previous char was lowercase, OR
            // - The previous char was uppercase and the next char is lowercase
            //   (handles "HTTPSEnabled" → "https_enabled")
            if !at_start && (prev_lower || (prev_upper && next_lower)) {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            result.push(ch);
        }
    }

    // Handle leading digit
    if result.starts_with(|c: char| c.is_ascii_digit()) {
        result.insert(0, '_');
    }

    result
}

/// Escape Rust keywords by appending `_`.
pub fn escape_keyword(s: &str) -> String {
    if RUST_KEYWORDS.contains(&s) {
        format!("{s}_")
    } else {
        s.to_string()
    }
}

/// Convert a camelCase property name to its Pulumi wire name.
/// This is the identity function — Pulumi already uses camelCase on the wire.
pub fn field_to_wire_name(rust_field: &str) -> String {
    // Reverse of camel_to_snake — but it's simpler to just store the original name
    // in generated code. This is a placeholder for completeness.
    rust_field.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kebab_to_snake() {
        assert_eq!(kebab_to_snake("azure-native"), "azure_native");
        assert_eq!(kebab_to_snake("random"), "random");
        assert_eq!(kebab_to_snake("google-cloud"), "google_cloud");
    }

    #[test]
    fn test_camel_to_snake_basic() {
        assert_eq!(camel_to_snake("resourceGroupName"), "resource_group_name");
        assert_eq!(camel_to_snake("location"), "location");
        assert_eq!(camel_to_snake("VirtualNetwork"), "virtual_network");
    }

    #[test]
    fn test_camel_to_snake_acronyms() {
        assert_eq!(camel_to_snake("getHTTPSEnabled"), "get_https_enabled");
        assert_eq!(camel_to_snake("enableSSL"), "enable_ssl");
        assert_eq!(camel_to_snake("XMLParser"), "xml_parser");
        assert_eq!(camel_to_snake("AWSRegion"), "aws_region");
    }

    #[test]
    fn test_camel_to_snake_edge_cases() {
        assert_eq!(camel_to_snake(""), "");
        assert_eq!(camel_to_snake("a"), "a");
        assert_eq!(camel_to_snake("A"), "a");
        assert_eq!(camel_to_snake("ID"), "id");
    }

    #[test]
    fn test_escape_keyword() {
        assert_eq!(escape_keyword("type"), "type_");
        assert_eq!(escape_keyword("self"), "self_");
        assert_eq!(escape_keyword("fn"), "fn_");
        assert_eq!(escape_keyword("name"), "name"); // not a keyword
        assert_eq!(escape_keyword("r#type"), "r#type"); // already escaped
    }

    #[test]
    fn test_naming_context_module_path() {
        let ctx = NamingContext::new("azure-native");
        assert_eq!(ctx.package_name, "azure_native");

        // Standard module
        assert_eq!(
            ctx.token_to_module_path("azure-native:resources:ResourceGroup"),
            vec!["resources"]
        );

        // Nested module with /
        assert_eq!(
            ctx.token_to_module_path("azure-native:storage/blobService:BlobService"),
            vec!["storage", "blob_service"]
        );

        // Index module maps to root
        assert_eq!(
            ctx.token_to_module_path("random:index:RandomString"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_naming_context_type_name() {
        let ctx = NamingContext::new("azure-native");

        assert_eq!(
            ctx.token_to_type_name("azure-native:resources:ResourceGroup"),
            "ResourceGroup"
        );
        assert_eq!(
            ctx.token_to_type_name("azure-native:network:VirtualNetwork"),
            "VirtualNetwork"
        );
    }

    #[test]
    fn test_naming_context_property_to_field() {
        let ctx = NamingContext::new("mypkg");

        assert_eq!(ctx.property_to_field_name("resourceGroupName"), "resource_group_name");
        assert_eq!(ctx.property_to_field_name("type"), "type_");
        assert_eq!(ctx.property_to_field_name("location"), "location");
        assert_eq!(ctx.property_to_field_name("enableHTTPS"), "enable_https");
    }

    #[test]
    fn test_numeric_leading_name() {
        assert_eq!(camel_to_snake("3DView"), "_3d_view");
    }

    #[test]
    fn test_token_to_module_index_with_slash() {
        let ctx = NamingContext::new("random");
        // real-world: "random:index/randomString:RandomString" — "index" is skipped
        assert_eq!(
            ctx.token_to_module_path("random:index/randomString:RandomString"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_token_to_module_providers_with_type_prefix() {
        let ctx = NamingContext::new("random");
        // "pulumi:providers:random/terraformConfig" — type part prefix becomes module segment
        assert_eq!(
            ctx.token_to_module_path("pulumi:providers:random/terraformConfig"),
            vec!["providers", "random"]
        );
    }
}
