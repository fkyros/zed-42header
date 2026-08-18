# `zed-42header` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and package the `zed-42header` Zed extension and lightweight LSP server to insert and automatically update 42 standard file headers on save in Zed.

**Architecture:** A lightweight Rust native LSP server (`header42-lsp`) implementing `textDocument/codeAction` and `textDocument/formatting` paired with a sandboxed Zed WASM extension (`zed_extension_api`) that launches the LSP across multiple languages.

**Tech Stack:** Rust, `tower-lsp`, `tokio`, `zed_extension_api`, WebAssembly (`wasm32-wasip2`), GitHub Actions.

**Spec:** [`docs/superpowers/specs/2026-08-18-zed-42header-design.md`](file:///Users/gabri/mV/proyectos/fkyros/docs/superpowers/specs/2026-08-18-zed-42header-design.md)

## Global Constraints
- Strictly 11 lines per 42 header.
- 80 characters wide (79 characters for Python `#`).
- 5-space margins on text lines.
- Timestamps formatted strictly as `YYYY/MM/DD HH:MM:SS`.
- Identity resolution hierarchy: Zed `settings.json` -> `$USER42`/`$MAIL42` -> `$USER`/`$MAIL` -> System fallback (`<user>@student.42.fr`).
- Every task ends with tests passing and a local Git commit.

---

### Task 1: Initialize Git Repo & Workspace Scaffolding

**Files:**
- Create: `zed-42header/.gitignore`
- Create: `zed-42header/LICENSE`
- Create: `zed-42header/README.md`
- Create: `zed-42header/extension.toml`
- Create: `zed-42header/Cargo.toml`
- Create: `zed-42header/src/Cargo.toml`
- Create: `zed-42header/server/Cargo.toml`

**Interfaces:**
- Workspace root linking members `["src", "server"]`.

- [ ] **Step 1: Initialize git repository in `zed-42header`**
- [ ] **Step 2: Create root `.gitignore` and `LICENSE` (MIT)**
- [ ] **Step 3: Create `extension.toml` manifest**
- [ ] **Step 4: Create root workspace `Cargo.toml`, `src/Cargo.toml`, and `server/Cargo.toml`**
- [ ] **Step 5: Verify workspace build configuration with `cargo check`**
- [ ] **Step 6: Commit**

```bash
git add .
git commit -m "chore: initialize zed-42header workspace and extension manifest"
```

---

### Task 2: Language Comment Syntaxes & Configuration Module

**Files:**
- Create: `zed-42header/server/src/comments.rs`
- Create: `zed-42header/server/src/config.rs`
- Test: `zed-42header/server/tests/test_comments_and_config.rs`

**Interfaces:**
- `comments::get_delimiters(file_path: &str) -> CommentDelimiters { start, end, fill, max_width }`
- `config::resolve_identity(settings_user: Option<&str>, settings_mail: Option<&str>) -> (String, String)`

- [ ] **Step 1: Write unit tests for delimiter resolution across all 42 languages (C, Python, Shell, HTML, etc.) and identity resolution**
- [ ] **Step 2: Run tests to verify failure**
- [ ] **Step 3: Implement `comments.rs` and `config.rs`**
- [ ] **Step 4: Run tests to verify they pass**
- [ ] **Step 5: Commit**

```bash
git add server/
git commit -m "feat(server): add language comment matrix and identity resolution"
```

---

### Task 3: 42 Header Generation, Detection & Update Engine

**Files:**
- Create: `zed-42header/server/src/header.rs`
- Test: `zed-42header/server/tests/test_header_engine.rs`

**Interfaces:**
- `header::generate_header(filename: &str, user: &str, mail: &str, created: &str, updated: &str, delimiters: &CommentDelimiters) -> String`
- `header::detect_header(content: &str, delimiters: &CommentDelimiters) -> Option<HeaderInfo>`
- `header::update_header_timestamp(content: &str, user: &str, delimiters: &CommentDelimiters) -> Option<String>`

- [ ] **Step 1: Write unit tests for exact 80-col (and 79-col Python) formatting, ASCII art alignment, and update-on-save replacements**
- [ ] **Step 2: Run tests to verify failure**
- [ ] **Step 3: Implement `header.rs`**
- [ ] **Step 4: Run tests to verify they pass**
- [ ] **Step 5: Commit**

```bash
git add server/
git commit -m "feat(server): implement 42 standard header generation and parser"
```

---

### Task 4: Native LSP Server Implementation

**Files:**
- Create: `zed-42header/server/src/main.rs`
- Test: `zed-42header/server/tests/test_lsp_server.rs`

**Interfaces:**
- LSP Server handling `initialize`, `textDocument/codeAction` (`Insert 42 Header`, `Update 42 Header`), and `textDocument/formatting` (auto-update on save).

- [ ] **Step 1: Implement `tower-lsp` LanguageServer backend in `main.rs`**
- [ ] **Step 2: Add integration test sending JSON-RPC requests over channel/stdio**
- [ ] **Step 3: Run integration test to verify Code Action and Formatting responses**
- [ ] **Step 4: Commit**

```bash
git add server/
git commit -m "feat(server): implement LSP server with CodeActions and Formatting hooks"
```

---

### Task 5: WASM Extension Bridge

**Files:**
- Create: `zed-42header/src/lib.rs`
- Modify: `zed-42header/extension.toml`

**Interfaces:**
- Implements `zed::Extension` trait: finds or downloads `header42-lsp` binary and executes it for registered languages.

- [ ] **Step 1: Implement `zed_extension_api` extension struct and binary launcher in `src/lib.rs`**
- [ ] **Step 2: Test WASM build targeting `wasm32-wasip2`**
- [ ] **Step 3: Commit**

```bash
git add src/ extension.toml
git commit -m "feat(wasm): implement Zed WASM extension adapter"
```

---

### Task 6: GitHub Actions Multi-Platform Release CI & Documentation

**Files:**
- Create: `zed-42header/.github/workflows/release.yml`
- Create: `zed-42header/README.md`
- Create: `zed-42header/languages/c/snippets.json`

- [ ] **Step 1: Create GitHub Actions workflow for cross-compiling release binaries (macOS Intel/ARM, Linux x86_64/ARM64)**
- [ ] **Step 2: Write comprehensive user documentation with installation, keybindings, and settings guide**
- [ ] **Step 3: Commit**

```bash
git add .github/ README.md languages/
git commit -m "docs: add release workflow, README, and Zed keymap instructions"
```

---

### Task 7: Full End-to-End Verification & Dev Extension Test

- [ ] **Step 1: Run full test suite across workspace (`cargo test --workspace`)**
- [ ] **Step 2: Build release binaries and verify WASM bundle**
- [ ] **Step 3: Commit and tag initial version**
