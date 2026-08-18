# 42 Header for Zed

A native port of the official [`42paris/42header`](https://github.com/42paris/42header) plugin for the [Zed text editor](https://zed.dev).

Automatically inserts the standardized 42 Network 11-line file header and updates the timestamp on save.

## Features

- **Standard 42 Header**: Strict adherence to the 42 standard (11 lines, 80 columns, 79 for Python, 5-space margins, official ASCII art).
- **Auto-Update on Save**: Automatically updates the `Updated: YYYY/MM/DD HH:MM:SS by <user>` line when saving.
- **Multi-Language Support**: C, C++, Rust, Python, Go, JavaScript, TypeScript, HTML, CSS, Lua, Makefile, Shell scripts, Assembly, OCaml, and more.
- **Identity Resolution**: Configurable via Zed `settings.json`, with seamless fallback to `$USER42`/`$MAIL42`, `$USER`/`$MAIL`, or system defaults.
- **Code Actions & Keybindings**: Insert or update headers via Code Actions (`Cmd+.` / `Ctrl+.`) or bind directly to `<F1>`.

## Installation

1. Open Zed.
2. Open the Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`) and type `zed: extensions`.
3. Search for **42 Header** and click **Install**.

## Configuration

In your Zed `settings.json` (`~/.config/zed/settings.json` on macOS/Linux):

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

If not specified in `settings.json`, `zed-42header` will automatically look for `$USER42`/`$MAIL42`, followed by `$USER`/`$MAIL`.

## Keybindings

To trigger header insertion with `<F1>` like in Vim, add the following to your `keymap.json`:

```json
[
  {
    "context": "Editor",
    "bindings": {
      "f1": "editor::ToggleCodeActions"
    }
  }
]
```

## License

MIT License. See [LICENSE](LICENSE) for details.
