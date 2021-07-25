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
        include_str!("../vendor/tree-sitter-go-eb306e6e60f393df346cfc8cbfaf52667a37128a/queries/highlights.scm"),
        "",
        ""
    )
}

pub fn c() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_c() },
        include_str!(
            "../vendor/tree-sitter-c-008008e30a81849fca0c79291e2b480855e0e02c/queries/highlights.scm"
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
    HighlightConfiguration::new(
        unsafe { tree_sitter_toml() },
        include_str!("../vendor/tree-sitter-toml-e1aa4dd51bfa83fbde26565e0b80f5ed17b0bdc8/queries/highlights.scm"),
        "",
        "",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_go() {
        go().unwrap();
    }

    #[test]
    fn build_c() {
        c().unwrap();
    }

    #[test]
    fn build_rust() {
        rust().unwrap();
    }

    #[test]
    fn build_javascript() {
        javascript().unwrap();
    }

    #[test]
    fn build_java() {
        java().unwrap();
    }

    #[test]
    fn build_typescript() {
        typescript().unwrap();
    }

    #[test]
    fn build_tsx() {
        tsx().unwrap();
    }

    #[test]
    fn build_toml() {
        toml().unwrap();
    }
}
