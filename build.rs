use std::path::PathBuf;

fn main() {
    {
        let dir: PathBuf = [
            "vendor",
            "tree-sitter-go-eb306e6e60f393df346cfc8cbfaf52667a37128a",
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
            "tree-sitter-c-008008e30a81849fca0c79291e2b480855e0e02c",
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

    {
        let dir: PathBuf = [
            "vendor",
            "tree-sitter-rust-a360da0a29a19c281d08295a35ecd0544d2da211",
            "src",
        ]
        .iter()
        .collect();

        cc::Build::new()
            .warnings(false)
            .include(&dir)
            .file(dir.join("parser.c"))
            .file(dir.join("scanner.c"))
            .compile("tree-sitter-rust")
    }

    {
        let dir: PathBuf = [
            "vendor",
            "tree-sitter-javascript-2c5b138ea488259dbf11a34595042eb261965259",
            "src",
        ]
        .iter()
        .collect();

        cc::Build::new()
            .warnings(false)
            .include(&dir)
            .file(dir.join("parser.c"))
            .file(dir.join("scanner.c"))
            .compile("tree-sitter-javascript")
    }

    {
        let dir: PathBuf = [
            "vendor",
            "tree-sitter-java-2efe37f92d2e6aeb25186e9da07455bb4a30163c",
            "src",
        ]
        .iter()
        .collect();

        cc::Build::new()
            .warnings(false)
            .include(&dir)
            .file(dir.join("parser.c"))
            .compile("tree-sitter-java")
    }

    {
        let dir: PathBuf = [
            "vendor",
            "tree-sitter-typescript-ebd10b475722d59a1fa7e4b38047e768413794fc",
            "typescript",
            "src",
        ]
        .iter()
        .collect();

        cc::Build::new()
            .warnings(false)
            .include(&dir)
            .file(dir.join("parser.c"))
            .file(dir.join("scanner.c"))
            .compile("tree-sitter-typescript")
    }

    {
        let dir: PathBuf = [
            "vendor",
            "tree-sitter-typescript-ebd10b475722d59a1fa7e4b38047e768413794fc",
            "tsx",
            "src",
        ]
        .iter()
        .collect();

        cc::Build::new()
            .warnings(false)
            .include(&dir)
            .file(dir.join("parser.c"))
            .file(dir.join("scanner.c"))
            .compile("tree-sitter-tsx")
    }

    {
        let dir: PathBuf = [
            "vendor",
            "tree-sitter-toml-e1aa4dd51bfa83fbde26565e0b80f5ed17b0bdc8",
            "src",
        ]
        .iter()
        .collect();

        cc::Build::new()
            .warnings(false)
            .include(&dir)
            .file(dir.join("parser.c"))
            .file(dir.join("scanner.c"))
            .compile("tree-sitter-toml")
    }
}
