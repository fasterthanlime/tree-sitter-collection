use tree_sitter::Language;

extern "C" {
    pub fn tree_sitter_go() -> Language;
    pub fn tree_sitter_c() -> Language;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_all() {
        unsafe {
            tree_sitter_go();
            tree_sitter_c();
        }
    }
}
