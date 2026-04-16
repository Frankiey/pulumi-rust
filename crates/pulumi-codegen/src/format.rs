//! Output formatting for generated Rust code.
//!
//! Uses `prettyplease` to format generated Rust source code without requiring
//! an external `rustfmt` binary.

/// Format generated Rust source code using prettyplease.
///
/// If the code cannot be parsed by `syn`, returns the original code unchanged.
/// This is a best-effort formatter — generated code should be syntactically
/// valid, but we gracefully degrade rather than failing.
pub fn format_code(code: &str) -> String {
    match syn::parse_file(code) {
        Ok(syntax_tree) => prettyplease::unparse(&syntax_tree),
        Err(_) => code.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_ugly_code() {
        let ugly = r#"pub struct Foo{pub name:String,pub count:i64,}"#;
        let formatted = format_code(ugly);
        assert!(formatted.contains("pub struct Foo"));
        // prettyplease should add line breaks
        assert!(formatted.contains('\n'));
    }

    #[test]
    fn test_format_invalid_code_returns_original() {
        let invalid = "this is not valid rust {{{";
        let result = format_code(invalid);
        assert_eq!(result, invalid);
    }

    #[test]
    fn test_format_preserves_semantics() {
        let code = r#"
pub fn hello() -> String {
    "world".to_string()
}
"#;
        let formatted = format_code(code);
        assert!(formatted.contains("pub fn hello()"));
        assert!(formatted.contains("\"world\""));
    }
}
