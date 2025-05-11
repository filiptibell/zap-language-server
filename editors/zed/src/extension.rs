#![allow(clippy::unused_self)]

use zed_extension_api::{self as zed, LanguageServerId, Result};

mod constants;
mod fs;
mod github;
mod strings;

use self::constants::BINARY_NAME;
use self::fs::{cleanup_dir_entries, create_dir_if_nonexistent, file_exists};
use self::strings::PlatformStrings;

struct ZapExtension {
    cached_binary_path: Option<String>,
}

impl ZapExtension {
    fn language_server_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // Prefer a local binary, if one exists, or a cached one

        if let Some(path) = worktree.which(BINARY_NAME) {
            return Ok(path);
        } else if let Some(path) = self.cached_binary_path.clone().filter(|p| file_exists(p)) {
            return Ok(path);
        }

        // No local or cached binary exists, check for the latest released binary

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let pstrings = PlatformStrings::current();
        let (release_version, release_download_url) = github::find_latest_release(&pstrings)?;

        // Latest released binary was found, check if that's a version we have downloaded already

        let root_dir = pstrings.server_binary_root();
        let version_dir_name = pstrings.server_binary_dir(&release_version);
        let version_dir_path = format!("{root_dir}/{version_dir_name}");
        let binary_path = format!("{version_dir_path}/{}", pstrings.server_binary_path());

        if !file_exists(&binary_path) {
            // We don't have the latest binary, download it and get rid of old binaries

            create_dir_if_nonexistent(&root_dir)?;

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &release_download_url,
                &version_dir_path,
                zed::DownloadedFileType::Zip,
            )?;

            zed::make_file_executable(&binary_path)?;

            cleanup_dir_entries(&root_dir, &version_dir_name)?;
        }

        // We now know that we have a latest binary downloaded,
        // cache it for the remainder of the Zed editor session

        self.cached_binary_path = Some(binary_path.clone());

        Ok(binary_path)
    }

    fn language_server_args(&self) -> Vec<String> {
        vec![String::from("serve")]
    }
}

impl zed::Extension for ZapExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_path(language_server_id, worktree)?,
            args: self.language_server_args(),
            env: vec![],
        })
    }
}

zed::register_extension!(ZapExtension);
