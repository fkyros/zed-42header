# Architectural Design Specification: `zed-42header`

**Author:** Gabri / 42 Community  
**Date:** 2026-08-18  
**Status:** Approved for Implementation  
**Target Platform:** Zed Editor (macOS, Linux, Windows)  

---

## 1. Overview & Objectives

The goal of `zed-42header` is to provide 42 Network students with full, native parity with the official [`42paris/42header`](https://github.com/42paris/42header) Vim plugin inside the [Zed Editor](https://zed.dev).

### Core Goals:
1. **1-Click / Shortcut Insertion**: Insert standard 42 headers across all supported programming languages via Code Actions, Keybindings (`<F1>`), and Snippets.
2. **Automatic Timestamp Update on Save**: Automatically detect existing headers and update the `Updated: YYYY/MM/DD HH:MM:SS by <user>` line upon document save.
3. **Multi-tier Identity Resolution**: Seamlessly read user login and email from Zed `settings.json`, environment variables (`$USER42`, `$MAIL42`, `$USER`, `$MAIL`), or intelligent fallbacks (`<user>@student.42.fr`).
4. **Zero-Configuration End-User Experience**: Easily installable with 1-click from the Zed Extension Registry with automatic background binary distribution.

---

## 2. Architecture & Component Diagram

Zed extensions run in a WebAssembly sandbox (`wasm32-wasip2`) without direct APIs for arbitrary buffer mutation. To deliver high-performance code transformation and save-hooks, `zed-42header` adopts the standard **LSP Server + WASM Extension Bridge** pattern.

```
┌────────────────────────────────────────────────────────┐
│                      Zed Editor                        │
└──────┬───────────────────────▲──────────────────▲──────┘
       │ (1) Install & Launch  │ (4) LSP JSON-RPC │
       ▼                       │     CodeActions  │
┌─────────────────────────┐    │     Formatting   │
│   WASM Extension        │    │     Diagnostics  │
│  (zed_extension_api)    │    │                  │
│  - Finds/downloads LSP  │    │                  │
│  - Spawns LSP process   │    │                  │
└──────────────┬──────────┘    │                  │
               │ (2) Spawns    │                  │
               ▼               ▼                  │
┌─────────────────────────────────────────────────┴──────┐
│            header42-lsp (Native Binary)                │
│  - Lightweight Rust binary (< 4 MB, 0ms startup)       │
│  - Identity Resolver (settings.json -> ENV -> fallback)│
│  - 42 Header Generator & Parser (80 cols, 11 lines)    │
│  - LSP Handler (tower-lsp / lsp-server)                │
└────────────────────────────────────────────────────────┘
```

---

## 3. Detailed Specifications

### 3.1 42 Header Standard Format

The header is composed of 11 lines, 80 characters wide (79 characters for Python `#`), with 5-character margins for text lines.

```c
/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   example.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: login <login@student.42.fr>                 +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2026/08/18 20:10:00 by login             #+#    #+#             */
/*   Updated: 2026/08/18 20:10:00 by login            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */
```

#### Line Anatomy:
- **Line 1**: `/* ************************************************************************** */` (Top border)
- **Line 2**: `/*                                                                            */` (Empty line)
- **Line 3**: `/*                                                        :::      ::::::::   */` (ASCII Art line 1)
- **Line 4**: `/*   <filename>                                         :+:      :+:    :+:   */` (Filename + ASCII Art line 2)
- **Line 5**: `/*                                                    +:+ +:+         +:+     */` (ASCII Art line 3)
- **Line 6**: `/*   By: <login> <<mail>>                            +#+  +:+       +#+        */` (Author + ASCII Art line 4)
- **Line 7**: `/*                                                +#+#+#+#+#+   +#+           */` (ASCII Art line 5)
- **Line 8**: `/*   Created: YYYY/MM/DD HH:MM:SS by <login>             #+#    #+#             */` (Created timestamp + ASCII Art line 6)
- **Line 9**: `/*   Updated: YYYY/MM/DD HH:MM:SS by <login>            ###   ########.fr       */` (Updated timestamp + ASCII Art line 7)
- **Line 10**: `/*                                                                            */` (Empty line)
- **Line 11**: `/* ************************************************************************** */` (Bottom border)

#### 7-Line ASCII Art Definition:
```
        :::      ::::::::
      :+:      :+:    :+:
    +:+ +:+         +:+  
  +#+  +:+       +#+     
+#+#+#+#+#+   +#+        
     #+#    #+#          
    ###   ########.fr    
```

### 3.2 Supported Languages & Comment Syntax Matrix

| File Extensions / Language IDs | Start Delimiter (`start`) | End Delimiter (`end`) | Fill Char (`fill`) | Line Max Width |
| :--- | :--- | :--- | :--- | :--- |
| `.c`, `.h`, `.cc`, `.hh`, `.cpp`, `.hpp`, `.tpp`, `.ipp`, `.cxx`, `.go`, `.rs`, `.php`, `.java`, `.kt`, `.kts`, `.css`, `.scss` | `/*` | `*/` | `*` | 80 |
| `.htm`, `.html`, `.xml` | `<!--` | `-->` | `*` | 80 |
| `.js`, `.ts`, `.jsx`, `.tsx` | `//` | `//` | `*` | 80 |
| `.py` | `#` | `#` | `*` | 79 |
| `.sh`, `.bash`, `.zsh`, `Makefile`, `.mk` | `#` | `#` | `*` | 80 |
| `.lua` | `--` | `--` | `-` | 80 |
| `.asm`, `.s` | `;` | `;` | `*` | 80 |
| `.tex` | `%` | `%` | `*` | 80 |
| `.ml`, `.mli`, `.mll`, `.mly` | `(*` | `*)` | `*` | 80 |
| `.vim`, `.vimrc` | `"` | `"` | `*` | 80 |
| `.el`, `emacs` | `;` | `;` | `*` | 80 |
| `.f90`, `.f95`, `.f03`, `.f`, `.for` | `!` | `!` | `/` | 80 |

---

### 3.3 Multi-tier Identity Resolution

When constructing the Author and Timestamps, identity fields are resolved in strict hierarchical order:

1. **Zed `settings.json`**:
   ```json
   {
     "lsp": {
       "header42-lsp": {
         "initialization_options": {
           "user": "custom_login",
           "mail": "custom_login@student.42.fr"
         }
       }
     }
   }
   ```
2. **42 Environment Variables**: `$USER42` and `$MAIL42`.
3. **Standard Environment Variables**: `$USER` (or `$USERNAME` on Windows) and `$MAIL`.
4. **Fallback**: Local OS user name + `<username>@student.42.fr`.

---

### 3.4 LSP Capabilities & Behaviors

The `header42-lsp` server handles standard LSP requests:

1. **`textDocument/codeAction`**:
   - Detects if lines 1–11 contain a valid 42 header.
   - If **missing**: Returns a Code Action:
     - Title: `Insert 42 Header`
     - Kind: `quickfix` / `source.organizeImports`
     - Edits: Insert the 11-line header at line `0`, character `0`.
   - If **present**: Returns a Code Action:
     - Title: `Update 42 Header`
     - Edits: Replaces line 9 (`Updated: ...`) with current timestamp and user.

2. **`textDocument/formatting`** (Auto-Update on Save):
   - When Zed formats on save, the LSP inspects lines 1–11.
   - If a valid 42 header is present, it computes the new line 9 and returns a single `TextEdit` targeting line 9, keeping the rest of the buffer untouched.

3. **`textDocument/didChange` & Diagnostics (Optional)**:
   - Can emit a soft `Information` / `Hint` diagnostic if a file has no header, allowing quick-fix invocation via `Cmd+.` / `Ctrl+.`.

---

## 4. Repository Layout

```
zed-42header/
├── .github/
│   └── workflows/
│       └── release.yml          # Builds & attaches cross-platform LSP binaries
├── Cargo.toml                  # Cargo workspace manifest
├── extension.toml              # Zed extension manifest
├── LICENSE                     # MIT / GPL License (Mandatory for marketplace)
├── README.md                   # Installation & usage documentation
├── languages/
│   └── c/
│       └── snippets.json       # Optional quick-snippets
├── src/                        # WASM Extension (compiled to wasm32-wasip2)
│   ├── Cargo.toml
│   └── lib.rs                  # Extension lifecycle & binary manager
└── server/                     # Standalone Native LSP Server
    ├── Cargo.toml
    └── src/
        ├── main.rs             # LSP Server main loop
        ├── header.rs           # Header generator and parser
        ├── comments.rs         # Filetype & delimiter resolver
        └── config.rs           # Identity resolution
```

---

## 5. User Configuration & Keybindings Guide

Users can configure their keybindings in Zed's `keymap.json` to mirror Vim:

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

And in `settings.json` to enable automatic update on save:
```json
{
  "format_on_save": "on",
  "lsp": {
    "header42-lsp": {
      "initialization_options": {
        "user": "myuser",
        "mail": "myuser@student.42.fr"
      }
    }
  }
}
```

---

## 6. Testing & Quality Assurance Plan

1. **Unit Testing (`server/src/header.rs`)**:
   - Validate strict 80-character width across all languages.
   - Validate 79-character width for Python (`#`).
   - Validate filename truncation for very long filenames.
   - Validate existing header detection regex.
   - Validate timestamp updating while preserving creation date.
2. **Integration Testing**:
   - Spin up `header42-lsp` over stdio and send LSP `initialize`, `textDocument/codeAction`, and `textDocument/formatting` JSON-RPC payloads.
3. **Local Zed Dev Installation**:
   - Compile WASM with `cargo build --target wasm32-wasip2`.
   - Install as Dev Extension via `zed: extensions` -> `Install Dev Extension`.
   - Verify `<F1>` / Code Action insertion and save timestamp update in real C/Python/Makefile files.

---

## 7. Marketplace Submission Plan

1. Release `header42-lsp` binaries via GitHub Actions for:
   - `x86_64-apple-darwin` (macOS Intel)
   - `aarch64-apple-darwin` (macOS Apple Silicon)
   - `x86_64-unknown-linux-gnu` (Linux x86_64)
   - `aarch64-unknown-linux-gnu` (Linux ARM64)
2. Submit PR to [zed-industries/extensions](https://github.com/zed-industries/extensions):
   - Add submodule: `extensions/42header`
   - Add entry to `extensions.toml`
   - Run `pnpm sort-extensions`
