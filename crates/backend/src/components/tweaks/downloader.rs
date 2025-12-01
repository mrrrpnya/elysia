use std::path::PathBuf;
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
    let temp_file = tweaks_dir.join("jade.tmp.zip");
    let mut file = fs::File::create(&temp_file).await?;
    file.write_all(&bytes).await?;

    tokio::task::spawn_blocking({
        let temp_file = temp_file.clone();
        let extract_path = extract_path.clone();
        move || -> Result<()> {
            let file = std::fs::File::open(&temp_file)?;
            let mut archive = zip::ZipArchive::new(file)?;
            archive.extract(&extract_path)?;
            Ok(())
        }
    })
    .await??;

    let _ = fs::remove_file(&temp_file).await;

    println!("Jadeite installed");
    Ok(extract_path)
}