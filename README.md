# 42 Header for Zed

A port of the standard 42 School file header for [Zed Editor](https://zed.dev).

Automatically inserts the 11-line 42 header with official ASCII art and updates the `Updated:` timestamp on save across all major languages used at 42.

---

## Installation & Setup

### 1. Install Extension in Zed
* **From Extension Registry**: Open Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`), run `zed: extensions`, search for **42 Header**, and click **Install**.
* **Local Dev**: Run `zed: install dev extension` and select this repository folder.

### 2. Configure `settings.json`
Open settings (`Cmd+,` or `Cmd+Shift+P` → `zed: open settings`) and add your 42 login and email:

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
  },
  "languages": {
    "C": { "language_servers": ["header42-lsp", "..."] },
    "C++": { "language_servers": ["header42-lsp", "..."] },
    "Python": { "language_servers": ["header42-lsp", "..."] },
    "Makefile": { "language_servers": ["header42-lsp", "..."] },
    "Bash": { "language_servers": ["header42-lsp", "..."] },
    "Shell Script": { "language_servers": ["header42-lsp", "..."] },
    "Rust": { "language_servers": ["header42-lsp", "..."] },
    "Go": { "language_servers": ["header42-lsp", "..."] },
    "JavaScript": { "language_servers": ["header42-lsp", "..."] },
    "TypeScript": { "language_servers": ["header42-lsp", "..."] }
  }
}
```

### 3. Keybindings (`keymap.json`)
Open keymap (`Cmd+Shift+P` → `zed: open keymap`) to bind `F1` (standard 42 Vim shortcut):

```json
[
  {
    "context": "Editor",
    "bindings": {
      "f1": "editor::ToggleCodeActions",
      "cmd-shift-h": "editor::ToggleCodeActions",
      "ctrl-shift-h": "editor::ToggleCodeActions"
    }
  }
]
```

---

## How to Use

1. **Insert Header**: Press `F1` (or `Cmd+.` / `Ctrl+.`) in any file and select **"Insert 42 Header"**.
2. **Auto-Update Timestamp**: Save the file (`Cmd+S` / `Ctrl+S`). The `Updated:` line updates automatically on every save.

---

## Supported Languages

* **C / C++ / Rust / Go / Java / Kotlin / PHP / CSS / SCSS**: `/* ... */` (80 columns)
* **Python**: `# ... #` (79 columns, PEP 8)
* **Shell / Makefile**: `# ... #` (80 columns)
* **JavaScript / TypeScript**: `// ... //` (80 columns)
* **HTML / XML**: `<!-- ... -->` (80 columns)
* **Lua**: `-- ... --` (80 columns)
* **Assembly**: `; ... ;` (80 columns)
* **OCaml**: `(* ... *)` (80 columns)
* **LaTeX**: `% ... %` (80 columns)
* **Fortran**: `! ... !` (80 columns)
* **Vimscript**: `" ... "` (80 columns)

---

## License

[MIT License](LICENSE) • Upstream 42 header formatting and ASCII art are ported from [`42Paris/42header`](https://github.com/42Paris/42header).

---

<sub>Built with **Google Antigravity** & **Gemini 3.7 Flash**</sub>

