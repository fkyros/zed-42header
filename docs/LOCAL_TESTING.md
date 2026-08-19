# Local Development & Verification Guide

This guide walks you through testing and verifying the `zed-42header` extension locally within [Zed Editor](https://zed.dev).

---

## Prerequisites

Ensure you have the following installed on your development machine:

1. **Rust Toolchain**: `rustc` and `cargo` (1.80+ recommended).
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **WASM Compilation Target**: Zed builds dev extensions targeting WASI.
   ```bash
   rustup target add wasm32-wasip2
   ```
3. **Zed Editor**: Stable or Preview build installed ([zed.dev](https://zed.dev)).

---

## Step 1: Build and Install Native LSP Binary

The extension delegates header generation and save formatting to `header42-lsp`, a standalone native binary.

> [!IMPORTANT]
> `header42-lsp` is an stdio LSP server that communicates via JSON-RPC over `stdin`/`stdout`. Do not run it interactively in a terminal expecting a CLI shell prompt; it will wait for JSON-RPC messages on `stdin`.

### 1. Build the Release Binary
From the root of the `zed-42header` repository:
```bash
cargo build --release -p header42-lsp
```

### 2. Verify Binary Creation
Check that the binary was created successfully:
```bash
test -f target/release/header42-lsp && echo "Binary build OK"
```

### 3. Install to PATH
Install the compiled binary into a directory present in your `$PATH` (such as `~/.cargo/bin` or `/usr/local/bin`):
```bash
cp target/release/header42-lsp ~/.cargo/bin/
```

Verify that your shell can locate it:
```bash
which header42-lsp
```

---

## Step 2: Install Dev Extension in Zed

1. Open **Zed**.
2. Open the Command Palette:
   - **macOS**: `Cmd+Shift+P`
   - **Linux / Windows**: `Ctrl+Shift+P`
3. Type `zed: extensions` and press `Enter`.
4. In the Extensions tab, click **Install Dev Extension** (or choose `extensions: install dev extension` from the Command Palette).
5. Select the root directory of the `zed-42header` repository.
6. Zed will compile the WASM extension using `wasm32-wasip2` and register the extension locally.

---

## Step 3: Configure Zed Settings & Keybindings

### 1. `settings.json` Configuration

Open your Zed settings (`Cmd+,` on macOS / `Ctrl+,` on Linux/Windows, or run `zed: open settings` in the Command Palette).

Add or update the following configuration:

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

> [!NOTE]
> If `user` or `mail` are omitted from `initialization_options`, `header42-lsp` will automatically resolve identity using the fallback chain:
> 1. `$USER42` / `$MAIL42` environment variables
> 2. `$USER` (or `%USERNAME%` on Windows) / `$MAIL` environment variables
> 3. `<local_user>` / `<local_user>@student.42.fr`

### 2. `keymap.json` Configuration (Vim `<F1>` Parity)

Open your Zed keymap (`zed: open keymap` in the Command Palette) and bind the Code Action toggle to `<F1>` (the standard 42 Vim keybinding) and `Cmd+Shift+H` / `Ctrl+Shift+H`:

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

## Step 4: Interactive Verification Matrix & Checklist

Test `zed-42header` against various file types and workflows using the matrix below:

| Language / Filetype | File Extension | Expected Comment Delimiters | Expected Width | Fill Char |
| :--- | :--- | :--- | :--- | :--- |
| **C / C++** | `.c`, `.h`, `.cpp`, `.hpp` | `/* ... */` | 80 columns | `*` |
| **Python** | `.py` | `# ... #` | **79 columns** (PEP 8 standard) | `*` |
| **Makefile / Shell** | `Makefile`, `.mk`, `.sh`, `.bash`, `.zsh` | `# ... #` | 80 columns | `*` |
| **Lua** | `.lua` | `-- ... --` | 80 columns | `-` |
| **JavaScript / TypeScript** | `.js`, `.ts`, `.jsx`, `.tsx` | `// ... //` | 80 columns | `*` |
| **HTML / XML** | `.html`, `.xml` | `<!-- ... -->` | 80 columns | `*` |
| **Assembly** | `.s`, `.asm` | `; ... ;` | 80 columns | `*` |
| **OCaml** | `.ml`, `.mli` | `(* ... *)` | 80 columns | `*` |
| **LaTeX** | `.tex` | `% ... %` | 80 columns | `*` |
| **Fortran** | `.f90`, `.f95`, `.for` | `! ... !` | 80 columns | `/` |
| **Vimscript** | `.vim`, `.vimrc` | `" ... "` | 80 columns | `*` |

---

### Step-by-Step Verification Checklist

- [ ] **Test 1: Header Insertion on Empty C File**
  1. In Zed, create a new file named `test.c`.
  2. Press `F1` (or `Cmd+.` / `Ctrl+.`).
  3. Select **Insert 42 Header**.
  4. **Verify**: An 11-line header is inserted at the top of the file.
  5. **Verify**: The first line is `/* ************************************************************************** */` (exact 80 columns width).
  6. **Verify**: Line 4 contains `test.c` aligned with the standard 42 ASCII art.
  7. **Verify**: Line 6 contains `By: your_42_login <your_42_login@student.42.fr>`.

- [ ] **Test 2: Header Insertion on Python File (79-column check)**
  1. In Zed, create a new file named `script.py`.
  2. Press `F1` and select **Insert 42 Header**.
  3. **Verify**: The header uses `#` delimiters.
  4. **Verify**: Top border is `# *************************************************************************** #` (exact **79 columns** width, adhering to 42 Python standard).

- [ ] **Test 3: Automatic Timestamp Update on Save**
  1. Open the previously created `test.c` file containing a valid 42 header.
  2. Note the timestamp on Line 9 (`Updated: YYYY/MM/DD HH:MM:SS by ...`).
  3. Modify any line in the body of `test.c` (e.g., add `int main() {}`).
  4. Wait at least 1–2 seconds, then save the file (`Cmd+S` / `Ctrl+S`).
  5. **Verify**: Line 9's timestamp updates to the current date and time.
  6. **Verify**: Line 8 (`Created: ...`) timestamp remains unchanged.
  7. **Verify**: No other lines in the file are modified or rearranged.

- [ ] **Test 4: Manual Header Update Code Action**
  1. In a file with an existing header, press `F1` (or `Cmd+.` / `Ctrl+.`).
  2. **Verify**: The code action menu displays **Update 42 Header**.
  3. Trigger the action and verify Line 9 updates immediately.

- [ ] **Test 5: Delimiter & Format Verification for Other Languages**
  1. Test `Makefile`: Delimiters `# ... #`, width 80.
  2. Test `test.lua`: Delimiters `-- ... --`, fill char `-`, width 80 (`-- -------------------------------------------------------------------------- --`).
  3. Test `app.ts`: Delimiters `// ... //`, width 80.
  4. Test `index.html`: Delimiters `<!-- ... -->`, width 80.

---

## Step 5: Troubleshooting & Inspecting Logs

### Checking Language Server Logs in Zed
If code actions do not appear or headers do not update on save:

1. Open the Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`).
2. Run `zed: open log` to view Zed's general log output.
3. Run `zed: open language server logs` and select `header42-lsp` to view communication between Zed and the LSP server.

### Verifying Language Server Status
In Zed's status bar (bottom right), check for active language servers on the current buffer:
- If `header42-lsp` is running, it will appear in the LSP indicators.
- If it is not running, ensure the buffer's language is declared in `extension.toml` or explicitly configured in `settings.json` under `languages.<LanguageName>.language_servers`.

### Common Issues

1. **`header42-lsp: command not found`**:
   - Ensure `~/.cargo/bin` is in your `$PATH` or copy `header42-lsp` to `/usr/local/bin`.
   - Restart Zed so it inherits the updated environment variables.

2. **Save does not update timestamp**:
   - Check that `"format_on_save": "on"` is enabled in your Zed `settings.json`.
   - Ensure the file already contains a valid 42 header within lines 1–11.
