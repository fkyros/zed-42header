use zed_extension_api as zed;

struct Header42Extension {
    cached_binary_path: Option<String>,
}

impl Header42Extension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        if let Some(path) = worktree.which("header42-lsp") {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        let (os, arch) = zed::current_platform();
        let asset_name = format!(
            "header42-lsp-{arch}-{os}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X8664 => "x86_64",
                zed::Architecture::X86 => "x86",
            },
            os = match os {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
                zed::Os::Windows => "pc-windows-msvc.exe",
            }
        );

        let binary_path = format!("header42-lsp-{}", asset_name);

        if !std::fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::CheckingForUpdate,
            );
            let release = zed::latest_github_release(
                "fkyros/zed-42header",
                zed::GithubReleaseOptions {
                    require_assets: true,
                    pre_release: false,
                },
            )?;

            let asset = release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name || asset.name.starts_with(&asset_name))
                .ok_or_else(|| {
                    format!("no release asset found for platform {asset_name}")
                })?;

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )?;

            zed::make_file_executable(&binary_path)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for Header42Extension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let binary_path = self
            .language_server_binary_path(language_server_id, worktree)
            .unwrap_or_else(|_| "header42-lsp".to_string());

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(Header42Extension);
