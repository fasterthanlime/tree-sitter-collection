use std::path::PathBuf;

fn main() {
    {
        let dir: PathBuf = [
            "vendor",
            "tree-sitter-go-c03e250fe4b4021b0a0c81cf63b143371987ad40",
            "src",
        ]
        .iter()
        .collect();

        cc::Build::new()
            .warnings(false)
            .include(&dir)
            .file(dir.join("parser.c"))
            .compile("tree-sitter-go")
    }

    {
        let dir: PathBuf = [
            "vendor",
            "tree-sitter-c-99151b1e9293c9e025498fee7e6691e1a52e1d03",
            "src",
        ]
        .iter()
        .collect();

        cc::Build::new()
            .warnings(false)
            .include(&dir)
            .file(dir.join("parser.c"))
            .compile("tree-sitter-c")
    }
}
