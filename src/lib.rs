use zed_extension_api as zed;

struct Header42Extension;

impl zed::Extension for Header42Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: "header42-lsp".to_string(),
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(Header42Extension);
