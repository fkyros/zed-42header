# 42 Header for Zed

Community-driven port of the standardized [`42Paris/42header`](https://github.com/42Paris/42header) plugin for the [Zed Editor](https://zed.dev).

`zed-42header` is an independent, community-driven port that automatically inserts the standardized 42 Network 11-line file header and updates timestamps on save across all major languages used in the 42 curriculum.

---

## Features

- **Strict Standard Compliance**: Produces headers formatted to exact 42 standards (11 lines, 80 columns width, 79 columns for Python `#`, 5-character margins, and official ASCII art).
- **Auto-Update on Save**: Hooks into Zed's document formatting to automatically update the `Updated: YYYY/MM/DD HH:MM:SS by <user>` line when saving.
- **Multi-tier Identity Resolution**:
  1. Zed `settings.json` (`initialization_options.user` and `initialization_options.mail`)
  2. 42 Environment Variables: `$USER42` and `$MAIL42`
  3. Standard Environment Variables: `$USER` (or `$USERNAME` on Windows) and `$MAIL`
  4. Fallback: `<user>@student.42.fr` or `marvin` / `marvin@student.42.fr`
- **Zero-Config Distribution**: The extension automatically locates your local `header42-lsp` binary or downloads pre-compiled native binaries for macOS, Linux, and Windows from GitHub Releases.
- **Language Agnostic**: Full comment delimiter support for C, C++, Rust, Go, Python, JavaScript, TypeScript, HTML, CSS, SCSS, Lua, Makefile, Shell scripts, Assembly, OCaml, LaTeX, Fortran, and Vimscript.

---

## Installation

### From Zed Extension Registry
1. Open Zed.
2. Open the Command Palette (`Cmd+Shift+P` on macOS / `Ctrl+Shift+P` on Linux/Windows).
3. Search for `zed: extensions`.
4. Find **42 Header** and click **Install**.

### Dev / Local Installation
```bash
# Clone the repository
git clone https://github.com/fkyros/zed-42header.git
cd zed-42header

# Build the LSP binary locally
cargo build --release -p header42-lsp

# Put the binary in your PATH or /usr/local/bin
cp target/release/header42-lsp ~/.cargo/bin/
```
Then in Zed, run `zed: extensions` -> `Install Dev Extension` and select the repository directory.

---

## Configuration

In your Zed `settings.json` (`~/.config/zed/settings.json` on macOS/Linux or `%APPDATA%\Zed\settings.json` on Windows):

```json
{
  "format_on_save": "on",
  "lsp": {
    "header42-lsp": {
      "initialization_options": {
        "user": "your_42_login",
        "mail": "your_42_login@student.42.fr"
      }
    }
  }
}
```

---

## Keybindings (Vim `<F1>` Parity)

To trigger the header code action with `<F1>` or custom shortcuts, add the following to your Zed `keymap.json`:

```json
[
  {
    "context": "Editor",
    "bindings": {
      "f1": "editor::ToggleCodeActions",
      "cmd-shift-h": "editor::ToggleCodeActions"
    }
  }
]
```

---

## Supported Languages & Delimiters

| File Types | Delimiters | Fill | Width |
| :--- | :--- | :--- | :--- |
| `.c`, `.h`, `.cpp`, `.hpp`, `.rs`, `.go`, `.java`, `.kt`, `.php`, `.css` | `/* ... */` | `*` | 80 |
| `.html`, `.htm`, `.xml` | `<!-- ... -->` | `*` | 80 |
| `.js`, `.ts`, `.jsx`, `.tsx` | `// ... //` | `*` | 80 |
| `.py` | `# ... #` | `*` | 79 |
| `.sh`, `.bash`, `.zsh`, `Makefile`, `.mk` | `# ... #` | `*` | 80 |
| `.lua` | `-- ... --` | `-` | 80 |
| `.asm`, `.s` | `; ... ;` | `*` | 80 |
| `.tex` | `% ... %` | `*` | 80 |
| `.ml`, `.mli` | `(* ... *)` | `*` | 80 |
| `.vim`, `.vimrc` | `" ... "` | `*` | 80 |
| `.f90`, `.f95`, `.for` | `! ... !` | `/` | 80 |

---

## Architecture

The extension consists of two components:
1. **WASM Extension Bridge (`src/lib.rs`)**: Runs sandboxed in Zed (`wasm32-wasip2`), downloads or locates the native server binary, and connects it to Zed's language server registry.
2. **`header42-lsp` (`server/`)**: Ultra-fast standalone Rust LSP binary (< 3 MB, instant cold-start) handling `textDocument/codeAction` and `textDocument/formatting`.

---

## License

This project is licensed under the [MIT License](LICENSE).

Upstream 42 header formatting and ASCII art are ported from [`42Paris/42header`](https://github.com/42Paris/42header), which is dedicated to the Public Domain.

