use std::path::PathBuf;

fn main() {
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
        .compile("tree-sitter-collection")
}
