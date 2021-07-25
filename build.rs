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
            "tree-sitter-rust-40620bf4097cbc9cea79504d7e877865df43a19e",
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
            "tree-sitter-javascript-c0b6dbc5c13fc344672febe4d08cd2fcccad82d1",
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
            "tree-sitter-java-d8703b9e34425f2d0adb2838fa0381ab7f11d9da",
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
