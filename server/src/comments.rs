use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentDelimiters {
    pub start: &'static str,
    pub end: &'static str,
    pub fill: char,
    pub max_width: usize,
}

pub const DEFAULT_DELIMITERS: CommentDelimiters = CommentDelimiters {
    start: "/*",
    end: "*/",
    fill: '*',
    max_width: 80,
};

pub fn get_delimiters(file_path: &str) -> CommentDelimiters {
    let path = Path::new(file_path);
    let file_name = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("")
        .to_lowercase();

    if file_name == "makefile"
        || file_name.starts_with("makefile.")
        || file_name.ends_with(".mk")
        || file_name.ends_with(".mak")
        || file_name == "gnumakefile"
        || file_name == "ocamlmakefile"
    {
        return CommentDelimiters {
            start: "#",
            end: "#",
            fill: '*',
            max_width: 80,
        };
    }

    if file_name == ".vimrc" || file_name.ends_with(".vim") {
        return CommentDelimiters {
            start: "\"",
            end: "\"",
            fill: '*',
            max_width: 80,
        };
    }

    if file_name == "emacs" || file_name.ends_with(".el") {
        return CommentDelimiters {
            start: ";",
            end: ";",
            fill: '*',
            max_width: 80,
        };
    }

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        // C-family, Rust, Go, PHP, Java, Kotlin, CSS
        "c" | "h" | "cc" | "hh" | "cpp" | "hpp" | "tpp" | "ipp" | "cxx" | "go" | "rs" | "php"
        | "java" | "kt" | "kts" | "css" | "scss" => CommentDelimiters {
            start: "/*",
            end: "*/",
            fill: '*',
            max_width: 80,
        },
        // HTML / XML
        "htm" | "html" | "xml" => CommentDelimiters {
            start: "<!--",
            end: "-->",
            fill: '*',
            max_width: 80,
        },
        // JavaScript / TypeScript
        "js" | "ts" | "jsx" | "tsx" => CommentDelimiters {
            start: "//",
            end: "//",
            fill: '*',
            max_width: 80,
        },
        // Python (79 columns max)
        "py" => CommentDelimiters {
            start: "#",
            end: "#",
            fill: '*',
            max_width: 79,
        },
        // Shell & scripts
        "sh" | "bash" | "zsh" => CommentDelimiters {
            start: "#",
            end: "#",
            fill: '*',
            max_width: 80,
        },
        // Lua
        "lua" => CommentDelimiters {
            start: "--",
            end: "--",
            fill: '-',
            max_width: 80,
        },
        // Assembly
        "asm" | "s" => CommentDelimiters {
            start: ";",
            end: ";",
            fill: '*',
            max_width: 80,
        },
        // LaTeX / TeX
        "tex" => CommentDelimiters {
            start: "%",
            end: "%",
            fill: '*',
            max_width: 80,
        },
        // OCaml
        "ml" | "mli" | "mll" | "mly" => CommentDelimiters {
            start: "(*",
            end: "*)",
            fill: '*',
            max_width: 80,
        },
        // Fortran
        "f90" | "f95" | "f03" | "f" | "for" => CommentDelimiters {
            start: "!",
            end: "!",
            fill: '/',
            max_width: 80,
        },
        _ => DEFAULT_DELIMITERS,
    }
}
