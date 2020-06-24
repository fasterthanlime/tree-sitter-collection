use tree_sitter::{Language, QueryError};
use tree_sitter_highlight::HighlightConfiguration;

extern "C" {
    fn tree_sitter_go() -> Language;
    fn tree_sitter_c() -> Language;
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_javascript() -> Language;
    fn tree_sitter_java() -> Language;
    fn tree_sitter_typescript() -> Language;
    fn tree_sitter_tsx() -> Language;
    fn tree_sitter_toml() -> Language;
}

pub fn go() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_go() },
        include_str!("../vendor/tree-sitter-go-c03e250fe4b4021b0a0c81cf63b143371987ad40/queries/highlights.scm"),
        "",
        ""
    )
}

pub fn c() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_c() },
        include_str!(
            "../vendor/tree-sitter-c-99151b1e9293c9e025498fee7e6691e1a52e1d03/queries/highlights.scm"
        ),
        "",
        "",
    )
}

pub fn rust() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_rust() },
        include_str!("../vendor/tree-sitter-rust-40620bf4097cbc9cea79504d7e877865df43a19e/queries/highlights.scm"),
        include_str!("../vendor/tree-sitter-rust-40620bf4097cbc9cea79504d7e877865df43a19e/queries/injections.scm"),
        ""
    )
}

pub fn javascript() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_javascript() },
        include_str!("../vendor/tree-sitter-javascript-c0b6dbc5c13fc344672febe4d08cd2fcccad82d1/queries/highlights.scm"),
        include_str!("../vendor/tree-sitter-javascript-c0b6dbc5c13fc344672febe4d08cd2fcccad82d1/queries/injections.scm"),
        include_str!("../vendor/tree-sitter-javascript-c0b6dbc5c13fc344672febe4d08cd2fcccad82d1/queries/locals.scm"),
    )
}

pub fn java() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_java() },
        include_str!("../vendor/tree-sitter-java-d8703b9e34425f2d0adb2838fa0381ab7f11d9da/queries/highlights.scm"),
        "",
        "",
    )
}

pub fn typescript() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_typescript() },
        include_str!("../vendor/tree-sitter-typescript-ebd10b475722d59a1fa7e4b38047e768413794fc/queries/highlights.scm"),
        "",
        include_str!("../vendor/tree-sitter-typescript-ebd10b475722d59a1fa7e4b38047e768413794fc/queries/locals.scm"),
    )
}

pub fn tsx() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_tsx() },
        include_str!("../vendor/tree-sitter-typescript-ebd10b475722d59a1fa7e4b38047e768413794fc/queries/highlights.scm"),
        "",
        include_str!("../vendor/tree-sitter-typescript-ebd10b475722d59a1fa7e4b38047e768413794fc/queries/locals.scm"),
    )
}

pub fn toml() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(unsafe { tree_sitter_toml() }, "", "", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_all() {
        go().unwrap();
        c().unwrap();
        rust().unwrap();
        javascript().unwrap();
        java().unwrap();
        typescript().unwrap();
        tsx().unwrap();
        toml().unwrap();
    }
}
