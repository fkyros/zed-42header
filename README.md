# 42 Header for Zed

A port of the standard 42 School file header for [Zed Editor](https://zed.dev).

Automatically inserts the 11-line 42 header with official ASCII art and updates the `Updated:` timestamp on save across all major languages used at 42.

---

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/fkyros/zed-42header.git
   ```

2. **Install in Zed**:
   * Open Zed.
   * Press `Cmd+Shift+P` (macOS) / `Ctrl+Shift+P` (Linux/Windows) and run **`zed: install dev extension`**.
   * Select the cloned `zed-42header` folder.

> [!NOTE]
> **Zero build tools needed**: When the extension starts, its WebAssembly bridge automatically fetches the matching pre-compiled `header42-lsp` server binary for your OS and CPU (macOS Apple Silicon/Intel, Linux, Windows) directly from this repo's [GitHub Releases](https://github.com/fkyros/zed-42header/releases). No Rust or Cargo installation is required.

---

## Configuration (Optional)

By default, the header uses `marvin` and `marvin@student.42.fr` if no configuration is provided.

To configure your own 42 login and email:
1. Open the Command Palette (`Cmd+Shift+P` on macOS / `Ctrl+Shift+P` on Linux/Windows) and select **`zed: open settings`** *(or click the `{}` icon in the top right of the Settings tab)* to open your `settings.json`.
2. Add your login and email, and enable `format_on_save`:

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
    "Make": { "language_servers": ["header42-lsp", "..."] },
    "Makefile": { "language_servers": ["header42-lsp", "..."] }
  }
}
```

> [!NOTE]
> **Why is `languages` needed in `settings.json`?**  
> Zed uses the Language Server Protocol (LSP) to update timestamps on save. For languages with built-in servers (like `clangd` for C/C++), Zed only runs its default server unless told to run `header42-lsp` alongside it using `"..."`.

---

## Keybindings (Optional)

To bind **`Cmd+Shift+H`** or **`F1`** (standard 42 Vim shortcut) for one-click header insertion:
1. Press `Cmd+Shift+P` (macOS) / `Ctrl+Shift+P` (Linux/Windows) and select **`zed: open keymap`**.
2. Paste the following into `keymap.json`:

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

1. **Insert Header**: Press **`Cmd+Shift+H`** or **`F1`** in any file and select **"Insert 42 Header"**.
2. **Auto-Update Timestamp**: Save the file (**`Cmd+S`** / **`Ctrl+S`**). The `Updated:` line on line 9 updates automatically on every save.

> [!TIP]
> **Troubleshooting**: If the header action does not appear after changing settings or loading the extension, open the Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`) and run **`editor: restart language server`** (or close and reopen the tab).

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

