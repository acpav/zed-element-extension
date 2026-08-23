use zed::LanguageServerId;
use zed_extension_api as zed;

/// GitHub repository that publishes the server binaries.
const SERVER_REPO: &str = "acpav/lsp-element-xbsl";
/// Release tag to download. Bump together with a new server release.
const SERVER_VERSION: &str = "v0.4.0";
const SERVER_BINARY: &str = "lsp-element-xbsl";

struct XbslExtension {
    cached_server_path: Option<String>,
}

impl XbslExtension {
    fn server_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        // Prefer a server installed on $PATH (e.g. cargo install or a release
        // binary the user placed manually).
        if let Some(path) = worktree.which(SERVER_BINARY) {
            return Ok(path);
        }

        if let Some(path) = self.cached_server_path.clone() {
            return Ok(path);
        }

        let binary_path = self.install_server(language_server_id)?;
        self.cached_server_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn install_server(&self, language_server_id: &LanguageServerId) -> zed::Result<String> {
        let target = server_target()?;
        let exe_suffix = if target == "x86_64-pc-windows-msvc" {
            ".exe"
        } else {
            ""
        };
        // Versioned directory so a future bump re-downloads instead of
        // reusing a stale binary.
        let server_dir = format!("lsp/{SERVER_BINARY}-{SERVER_VERSION}");
        let binary_path = format!("{server_dir}/{SERVER_BINARY}{exe_suffix}");

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        let release = zed::github_release_by_tag_name(SERVER_REPO, SERVER_VERSION).map_err(
            |error| format!("failed to fetch {SERVER_REPO} release {SERVER_VERSION}: {error}"),
        )?;
        let asset_name = format!("{SERVER_BINARY}-{target}.tar.gz");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("release {SERVER_VERSION} has no asset `{asset_name}`"))?;

        // `download_file` with `GzipTar` treats the path as the extraction
        // directory: the archive contents are unpacked directly into it.
        let download_result = zed::download_file(
            &asset.download_url,
            &server_dir,
            zed::DownloadedFileType::GzipTar,
        );
        if let Err(error) = download_result {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Failed(format!(
                    "failed to download {SERVER_BINARY}: {error}"
                )),
            );
            return Err(format!("failed to download {SERVER_BINARY}: {error}"));
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        if exe_suffix.is_empty() {
            zed::make_file_executable(&binary_path)?;
        }
        Ok(binary_path)
    }
}

/// Rust target triples published by the server release workflow.
fn server_target() -> zed::Result<String> {
    let (os, architecture) = zed::current_platform();
    match (os, architecture) {
        (zed::Os::Mac, zed::Architecture::Aarch64) => Ok("aarch64-apple-darwin".into()),
        (zed::Os::Mac, zed::Architecture::X8664) => Ok("x86_64-apple-darwin".into()),
        (zed::Os::Linux, zed::Architecture::X8664) => Ok("x86_64-unknown-linux-gnu".into()),
        (zed::Os::Windows, zed::Architecture::X8664) => Ok("x86_64-pc-windows-msvc".into()),
        _ => Err(format!(
            "no {SERVER_BINARY} {SERVER_VERSION} binary for {os:?}/{architecture:?}; \
             install the server manually and put `{SERVER_BINARY}` on $PATH"
        )),
    }
}

impl zed::Extension for XbslExtension {
    fn new() -> Self {
        Self {
            cached_server_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let server_path = self.server_path(language_server_id, worktree)?;
        Ok(zed::Command {
            command: server_path,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(XbslExtension);
