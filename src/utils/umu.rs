use std::fs;
use std::path::{Path, PathBuf};

use freya::prelude::{Readable, Signal};
use crate::settings::GlobalSettings;
use crate::utils::filesystem::ensure_dir;

const UMU_VERSION: &str = "1.2.9";

pub async fn setup_umu() -> Result<PathBuf, String> {
    let ctx = &dioxus::hooks::use_context::<Signal<GlobalSettings>>();
    let settings = &ctx.read();

    let umu_dir = settings.components_directory.join("umu-launcher");
    let umu_run = umu_dir.join("umu-run");

    if umu_run.exists() {
        return Ok(umu_run);
    }

    ensure_dir(&umu_dir)?;
    ensure_dir(&settings.temp_directory)?;

    let archive_path = download_umu(&settings.temp_directory).await?;
    extract_umu(&archive_path, &umu_dir)?;
    let _ = fs::remove_file(&archive_path);

    if !umu_run.exists() {
        return Err(format!("umu-run not found at {umu_run:?} after extraction"));
    }

    Ok(umu_run)
}

async fn download_umu(temp_dir: &Path) -> Result<PathBuf, String> {
    let url = format!(
        "https://github.com/Open-Wine-Components/umu-launcher/releases/download/{0}/umu-launcher-{0}-zipapp.tar",
        UMU_VERSION
    );

    let archive_path = temp_dir.join(format!("umu-launcher-{}-zipapp.tar", UMU_VERSION));

    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to download umu-launcher: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("Download failed with status: {}", resp.status()));
    }

    let bytes = resp.bytes()
        .await
        .map_err(|e| format!("Failed to read umu-launcher bytes: {e}"))?;

    fs::write(&archive_path, &bytes)
        .map_err(|e| format!("Failed to write archive to {archive_path:?}: {e}"))?;

    Ok(archive_path)
}

fn extract_umu(archive_path: &Path, dest_dir: &Path) -> Result<(), String> {
    let file = fs::File::open(archive_path)
        .map_err(|e| format!("Failed to open archive {archive_path:?}: {e}"))?;

    let mut archive = tar::Archive::new(file);
    archive.unpack(dest_dir)
        .map_err(|e| format!("Failed to extract archive to {dest_dir:?}: {e}"))?;

    Ok(())
}
