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
        include_str!("../vendor/tree-sitter-rust-a360da0a29a19c281d08295a35ecd0544d2da211/queries/highlights.scm"),
        include_str!("../vendor/tree-sitter-rust-a360da0a29a19c281d08295a35ecd0544d2da211/queries/injections.scm"),
        ""
    )
}

pub fn javascript() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_javascript() },
        include_str!("../vendor/tree-sitter-javascript-2c5b138ea488259dbf11a34595042eb261965259/queries/highlights.scm"),
        include_str!("../vendor/tree-sitter-javascript-2c5b138ea488259dbf11a34595042eb261965259/queries/injections.scm"),
        include_str!("../vendor/tree-sitter-javascript-2c5b138ea488259dbf11a34595042eb261965259/queries/locals.scm"),
    )
}

pub fn java() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_java() },
        include_str!("../vendor/tree-sitter-java-2efe37f92d2e6aeb25186e9da07455bb4a30163c/queries/highlights.scm"),
        "",
        "",
    )
}

pub fn typescript() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_typescript() },
        include_str!("../vendor/tree-sitter-typescript-fd08586b72f2ba8776d865b32aca561285c10cfd/queries/highlights.scm"),
        "",
        include_str!("../vendor/tree-sitter-typescript-fd08586b72f2ba8776d865b32aca561285c10cfd/queries/locals.scm"),
    )
}

pub fn tsx() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_tsx() },
        include_str!("../vendor/tree-sitter-typescript-fd08586b72f2ba8776d865b32aca561285c10cfd/queries/highlights.scm"),
        "",
        include_str!("../vendor/tree-sitter-typescript-fd08586b72f2ba8776d865b32aca561285c10cfd/queries/locals.scm"),
    )
}

pub fn toml() -> Result<HighlightConfiguration, QueryError> {
    HighlightConfiguration::new(
        unsafe { tree_sitter_toml() },
        include_str!("../vendor/tree-sitter-toml-8bd2056818b21860e3d756b5a58c4f6e05fb744e/queries/highlights.scm"),
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
