use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::Result;
use common::utils::filesystem::ensure_dir;
use serde::{Deserialize, Serialize};

use crate::globals::DATA_PATH;

/// Available runner types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunnerType {
    Wine,
    Proton,
}

impl RunnerType {
    pub fn directory_name(&self) -> &'static str {
        match self {
            RunnerType::Wine => "wine",
            RunnerType::Proton => "proton",
        }
    }
}

/// A runner that can be downloaded and installed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableRunner {
    pub name: String,
    pub display_name: String,
    pub runner_type: RunnerType,
    pub version: String,
    pub download_url: String,
    /// The folder name to use when extracting (stripped of top-level directory)
    pub folder_name: String,
}

/// Status of a runner download/installation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunnerStatus {
    NotInstalled,
    Downloading { progress: f32 },
    Installing,
    Installed,
    Error(String),
}

/// An installed runner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledRunner {
    pub name: String,
    pub display_name: String,
    pub runner_type: RunnerType,
    pub version: String,
    pub path: PathBuf,
}

/// Get the default available runners
pub fn get_available_runners() -> Vec<AvailableRunner> {
    vec![
        AvailableRunner {
            name: "wine-tkg-aagl".to_string(),
            display_name: "Wine-TKG AAGL".to_string(),
            runner_type: RunnerType::Wine,
            version: "v10.15-7".to_string(),
            download_url: "https://github.com/NelloKudo/Wine-Builds/releases/download/wine-tkg-aagl-v10.15-7/wine-tkg-aagl-v10.15-7-x86_64.tar.xz".to_string(),
            folder_name: "wine-tkg-aagl-v10.15-7".to_string(),
        },
        AvailableRunner {
            name: "dwproton".to_string(),
            display_name: "DW Proton".to_string(),
            runner_type: RunnerType::Proton,
            version: "10.0-9".to_string(),
            download_url: "https://dawn.wine/dawn-winery/dwproton/releases/download/dwproton-10.0-9/dwproton-10.0-9.tar.xz".to_string(),
            folder_name: "GE-Proton".to_string(),
        },
    ]
}

/// Get the components directory
pub fn get_components_directory() -> PathBuf {
    DATA_PATH.join("components")
}

/// Get list of installed runners by scanning the components directory
pub fn get_installed_runners() -> Vec<InstalledRunner> {
    let components_dir = get_components_directory();
    let mut runners = Vec::new();
    
    // Check wine directory
    let wine_dir = components_dir.join("wine");
    if wine_dir.exists() {
        if let Ok(entries) = fs::read_dir(&wine_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    // Check if it's a valid wine runner (has bin/wine)
                    if entry.path().join("bin/wine").exists() {
                        runners.push(InstalledRunner {
                            name: name.clone(),
                            display_name: name.clone(),
                            runner_type: RunnerType::Wine,
                            version: name.clone(),
                            path: entry.path(),
                        });
                    }
                }
            }
        }
    }
    
    // Check proton directory
    let proton_dir = components_dir.join("proton");
    if proton_dir.exists() {
        if let Ok(entries) = fs::read_dir(&proton_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    // Check if it's a valid proton runner (has proton script)
                    if entry.path().join("proton").exists() {
                        runners.push(InstalledRunner {
                            name: name.clone(),
                            display_name: name.clone(),
                            runner_type: RunnerType::Proton,
                            version: name.clone(),
                            path: entry.path(),
                        });
                    }
                }
            }
        }
    }
    
    runners
}

/// Check if a specific runner is installed
pub fn is_runner_installed(runner: &AvailableRunner) -> bool {
    let components_dir = get_components_directory();
    let runner_dir = components_dir
        .join(runner.runner_type.directory_name())
        .join(&runner.folder_name);
    
    match runner.runner_type {
        RunnerType::Wine => runner_dir.join("bin/wine").exists(),
        RunnerType::Proton => runner_dir.join("proton").exists(),
    }
}

/// Download and install a runner
pub async fn install_runner(runner: &AvailableRunner) -> Result<PathBuf> {
    let components_dir = get_components_directory();
    let runner_type_dir = components_dir.join(runner.runner_type.directory_name());
    let temp_dir = DATA_PATH.join("temp");
    
    ensure_dir(&runner_type_dir).map_err(|e| anyhow::anyhow!("{}", e))?;
    ensure_dir(&temp_dir).map_err(|e| anyhow::anyhow!("{}", e))?;
    
    // Download the archive
    println!("Downloading {} from {}...", runner.display_name, runner.download_url);
    let archive_path = download_runner(&runner.download_url, &temp_dir, &runner.folder_name).await?;
    
    // Extract with top-level directory stripped
    println!("Extracting {}...", runner.display_name);
    let dest_path = runner_type_dir.join(&runner.folder_name);
    extract_runner(&archive_path, &dest_path)?;
    
    // Clean up temp file
    let _ = fs::remove_file(&archive_path);
    
    println!("{} installed successfully to {:?}", runner.display_name, dest_path);
    Ok(dest_path)
}

/// Download a runner archive
async fn download_runner(url: &str, temp_dir: &Path, name: &str) -> Result<PathBuf> {
    let extension = if url.ends_with(".tar.gz") {
        ".tar.gz"
    } else if url.ends_with(".tar.xz") {
        ".tar.xz"
    } else {
        ".tar"
    };
    
    let archive_path = temp_dir.join(format!("{}{}", name, extension));
    
    let resp = reqwest::get(url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to download runner: {e}"))?;
    
    if !resp.status().is_success() {
        return Err(anyhow::anyhow!("Download failed with status: {}", resp.status()));
    }
    
    let bytes = resp.bytes()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read bytes: {e}"))?;
    
    fs::write(&archive_path, &bytes)
        .map_err(|e| anyhow::anyhow!("Failed to write archive: {e}"))?;
    
    Ok(archive_path)
}

/// Extract a runner archive, stripping the top-level directory
fn extract_runner(archive_path: &Path, dest_path: &Path) -> Result<()> {
    let archive_name = archive_path.to_string_lossy();
    
    // Create destination directory
    ensure_dir(dest_path).map_err(|e| anyhow::anyhow!("{}", e))?;
    
    if archive_name.ends_with(".tar.gz") {
        extract_tar_gz(archive_path, dest_path)?;
    } else if archive_name.ends_with(".tar.xz") {
        extract_tar_xz(archive_path, dest_path)?;
    } else {
        extract_tar(archive_path, dest_path)?;
    }
    
    Ok(())
}

/// Extract a .tar.gz archive, stripping top-level directory
fn extract_tar_gz(archive_path: &Path, dest_path: &Path) -> Result<()> {
    let file = fs::File::open(archive_path)?;
    let decoder = flate2::read::GzDecoder::new(file);
    extract_tar_from_reader(decoder, dest_path)
}

/// Extract a .tar.xz archive, stripping top-level directory
fn extract_tar_xz(archive_path: &Path, dest_path: &Path) -> Result<()> {
    let file = fs::File::open(archive_path)?;
    let decoder = xz2::read::XzDecoder::new(file);
    extract_tar_from_reader(decoder, dest_path)
}

/// Extract a plain .tar archive, stripping top-level directory
fn extract_tar(archive_path: &Path, dest_path: &Path) -> Result<()> {
    let file = fs::File::open(archive_path)?;
    extract_tar_from_reader(file, dest_path)
}

/// Extract from a tar reader, stripping the top-level directory
fn extract_tar_from_reader<R: Read>(reader: R, dest_path: &Path) -> Result<()> {
    let mut archive = tar::Archive::new(reader);
    
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        
        // Strip the top-level directory
        let stripped_path: PathBuf = path.components().skip(1).collect();
        
        if stripped_path.as_os_str().is_empty() {
            continue;
        }
        
        let full_path = dest_path.join(&stripped_path);
        
        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Extract based on entry type
        let entry_type = entry.header().entry_type();
        match entry_type {
            tar::EntryType::Directory => {
                fs::create_dir_all(&full_path)?;
            }
            tar::EntryType::Regular => {
                let mut file = fs::File::create(&full_path)?;
                std::io::copy(&mut entry, &mut file)?;
                
                // Preserve executable permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(mode) = entry.header().mode() {
                        let permissions = fs::Permissions::from_mode(mode);
                        let _ = fs::set_permissions(&full_path, permissions);
                    }
                }
            }
            tar::EntryType::Symlink => {
                #[cfg(unix)]
                {
                    if let Ok(link_target) = entry.link_name() {
                        if let Some(target) = link_target {
                            let _ = std::os::unix::fs::symlink(&target, &full_path);
                        }
                    }
                }
            }
            tar::EntryType::Link => {
                #[cfg(unix)]
                {
                    if let Ok(link_target) = entry.link_name() {
                        if let Some(target) = link_target {
                            // Hard links - strip top level from target too
                            let stripped_target: PathBuf = target.components().skip(1).collect();
                            let target_full = dest_path.join(&stripped_target);
                            let _ = fs::hard_link(&target_full, &full_path);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    Ok(())
}

/// Delete an installed runner
pub fn delete_runner(runner: &InstalledRunner) -> Result<()> {
    if runner.path.exists() {
        fs::remove_dir_all(&runner.path)?;
    }
    Ok(())
}
