use std::path::PathBuf;
use std::process::Command;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use anyhow::Result;

pub async fn jade_download(tweaks_dir: PathBuf) -> Result<PathBuf> {
    let extract_path = tweaks_dir.join("jadeite");

    if extract_path.exists() {
        println!("Jadeite already exists");
        return Ok(extract_path);
    }

    println!("Downloading Jadeite...");

    let url = "https://codeberg.org/mkrsym1/jadeite/releases/download/v5.0.1/v5.0.1.zip";

    // Use a client that follows redirects
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;
    
    let response = client.get(url).send().await?;
    
    // Check if the request was successful
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to download Jadeite: HTTP {}", response.status()));
    }
    
    let bytes = response.bytes().await?;
    
    // Verify we got a non-empty response
    if bytes.is_empty() {
        return Err(anyhow::anyhow!("Downloaded file is empty"));
    }

    fs::create_dir_all(&tweaks_dir).await?;
    fs::create_dir_all(&extract_path).await?;
    let temp_file = tweaks_dir.join("jade.tmp.zip");
    let mut file = fs::File::create(&temp_file).await?;
    file.write_all(&bytes).await?;

    // Use system unzip command which handles more zip formats
    let temp_file_clone = temp_file.clone();
    let extract_path_clone = extract_path.clone();
    tokio::task::spawn_blocking(move || -> Result<()> {
        let output = Command::new("unzip")
            .arg("-o") // overwrite without prompting
            .arg("-q") // quiet mode
            .arg(&temp_file_clone)
            .arg("-d")
            .arg(&extract_path_clone)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to extract zip: {}", stderr));
        }
        Ok(())
    })
    .await??;

    let _ = fs::remove_file(&temp_file).await;

    println!("Jadeite installed");
    Ok(extract_path)
}