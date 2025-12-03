// Video decoder module for streaming video frames
// Decodes video URLs frame-by-frame and sends them to Flutter

use anyhow::{Context, Result};
use bytes::Bytes;
use tokio::sync::mpsc;
use std::path::PathBuf;

/// Get the video cache directory path
fn get_video_cache_dir() -> Result<PathBuf> {
    let cache_dir = dirs::data_local_dir()
        .context("Failed to get local data directory")?
        .join("elysia")
        .join("image_cache"); // Reuse existing image_cache directory
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&cache_dir)
        .context("Failed to create cache directory")?;
    
    Ok(cache_dir)
}

/// Generate cache filename from URL
fn get_cache_filename(url: &str) -> String {
    // Use MD5 hash of URL as filename to avoid filesystem issues
    let hash = md5::compute(url.as_bytes());
    format!("{:x}.webm", hash)
}

/// Video frame data
#[derive(Clone, Debug)]
pub struct VideoFrame {
    /// RGBA image data
    pub data: Vec<u8>,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// Frame timestamp in milliseconds
    pub timestamp_ms: i64,
}

/// Video decoder that streams frames
pub struct VideoDecoder {
    url: String,
    frame_tx: mpsc::Sender<VideoFrame>,
    should_loop: bool,
}

impl VideoDecoder {
    /// Create a new video decoder
    pub fn new(url: String, frame_tx: mpsc::Sender<VideoFrame>) -> Self {
        Self { 
            url, 
            frame_tx,
            should_loop: true, // Videos loop by default
        }
    }

    /// Start decoding video and streaming frames with looping
    pub async fn start(self) -> Result<()> {
        // Download or get cached video
        let video_data = Self::get_or_download_video(&self.url).await?;
        
        // Decode frames in a blocking task since ffmpeg operations are CPU-bound
        let frame_tx = self.frame_tx;
        let should_loop = self.should_loop;
        tokio::task::spawn_blocking(move || {
            loop {
                if let Err(e) = Self::decode_frames(&video_data, frame_tx.clone()) {
                    eprintln!("Video decoding error: {}", e);
                    break;
                }
                
                // Check if we should loop
                if !should_loop || frame_tx.is_closed() {
                    break;
                }
                
                // Continue looping immediately without redownloading
                println!("Looping video playback...");
            }
        });

        Ok(())
    }
    
    /// Get video from cache or download it
    async fn get_or_download_video(url: &str) -> Result<Vec<u8>> {
        // Check disk cache
        let cache_dir = get_video_cache_dir()?;
        let cache_file = cache_dir.join(get_cache_filename(url));
        
        if cache_file.exists() {
            match std::fs::read(&cache_file) {
                Ok(cached_data) => {
                    println!("Using disk cached video: {} bytes from {:?}", cached_data.len(), cache_file);
                    return Ok(cached_data);
                }
                Err(e) => {
                    eprintln!("Failed to read cached video file: {}", e);
                    // Continue to download
                }
            }
        }
        
        // Download video
        println!("Downloading video from: {}", url);
        let video_data = Self::download_video(url).await?;
        
        // Store in disk cache
        if let Err(e) = std::fs::write(&cache_file, &video_data) {
            eprintln!("Failed to cache video to disk: {}", e);
        } else {
            println!("Cached video to disk: {:?}", cache_file);
        }
        
        Ok(video_data)
    }

    /// Download video from URL
    async fn download_video(url: &str) -> Result<Vec<u8>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .context("Failed to build HTTP client")?;
            
        let response = client
            .get(url)
            .send()
            .await
            .context("Failed to download video")?;

        // Check response status
        if !response.status().is_success() {
            anyhow::bail!("HTTP error: {}", response.status());
        }

        let bytes = response
            .bytes()
            .await
            .context("Failed to read video bytes")?;

        // Validate we have data
        if bytes.is_empty() {
            anyhow::bail!("Downloaded video is empty");
        }
        
        // Basic WebM/Matroska validation (starts with EBML header)
        if bytes.len() < 4 || &bytes[0..4] != b"\x1A\x45\xDF\xA3" {
            anyhow::bail!("Invalid video format: not a valid WebM/Matroska file");
        }

        println!("Downloaded video: {} bytes", bytes.len());
        Ok(bytes.to_vec())
    }

    /// Decode video frames using ffmpeg
    fn decode_frames(video_data: &[u8], frame_tx: mpsc::Sender<VideoFrame>) -> Result<()> {
        use ffmpeg_next as ffmpeg;
        use std::time::{Duration, Instant};

        // Initialize ffmpeg
        ffmpeg::init().context("Failed to initialize ffmpeg")?;
        
        // Suppress ffmpeg warnings for minor container issues
        ffmpeg::log::set_level(ffmpeg::log::Level::Error);

        // Create a temporary file for ffmpeg to read from
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("elysia_video_{}.webm", std::process::id()));
        std::fs::write(&temp_path, video_data).context("Failed to write temp video file")?;

        // Open input file with better error handling
        let mut ictx = ffmpeg::format::input(&temp_path)
            .context("Failed to open video file")?;

        // Find video stream
        let input = ictx
            .streams()
            .best(ffmpeg::media::Type::Video)
            .context("Could not find video stream")?;
        let video_stream_index = input.index();
        
        // Get frame rate
        let frame_rate = input.avg_frame_rate();
        let frame_duration = if frame_rate.0 > 0 && frame_rate.1 > 0 {
            Duration::from_secs_f64(frame_rate.1 as f64 / frame_rate.0 as f64)
        } else {
            // Default to 30fps if frame rate cannot be determined
            Duration::from_secs_f64(1.0 / 30.0)
        };
        
        println!("Video frame rate: {}/{} ({:.2} fps)", 
                 frame_rate.0, frame_rate.1, 
                 frame_rate.0 as f64 / frame_rate.1.max(1) as f64);

        // Create decoder
        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())
            .context("Failed to create codec context")?;
        let mut decoder = context_decoder
            .decoder()
            .video()
            .context("Failed to create video decoder")?;

        // Create scaler for RGBA conversion
        let mut scaler = ffmpeg::software::scaling::context::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            ffmpeg::format::Pixel::RGBA,
            decoder.width(),
            decoder.height(),
            ffmpeg::software::scaling::flag::Flags::BILINEAR,
        )
        .context("Failed to create scaler")?;

        // Decode frames with timing control
        let mut frame_count = 0;
        let time_base = input.time_base();
        let start_time = Instant::now();
        let mut last_frame_time = Instant::now();
        
        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                // Send packet to decoder, ignore minor errors
                if let Err(e) = decoder.send_packet(&packet) {
                    eprintln!("Warning: Error sending packet to decoder: {}", e);
                    continue;
                }
                
                let mut decoded = ffmpeg::util::frame::Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    // Rate limit: sleep to maintain target frame rate
                    let elapsed = last_frame_time.elapsed();
                    if elapsed < frame_duration {
                        std::thread::sleep(frame_duration - elapsed);
                    }
                    last_frame_time = Instant::now();
                    
                    let mut rgb_frame = ffmpeg::util::frame::Video::empty();
                    if let Err(e) = scaler.run(&decoded, &mut rgb_frame) {
                        eprintln!("Warning: Error scaling frame: {}", e);
                        continue;
                    }

                    // Convert to Vec<u8>
                    let data = rgb_frame.data(0).to_vec();
                    let width = rgb_frame.width();
                    let height = rgb_frame.height();
                    
                    // Calculate timestamp
                    let pts = decoded.pts().unwrap_or(0);
                    let timestamp_ms = (pts as f64 * f64::from(time_base) * 1000.0) as i64;

                    let video_frame = VideoFrame {
                        data,
                        width,
                        height,
                        timestamp_ms,
                    };

                    // Send frame (blocking with timeout)
                    if frame_tx.blocking_send(video_frame).is_err() {
                        // Receiver dropped, stop decoding
                        println!("Video stream closed by receiver");
                        let _ = std::fs::remove_file(&temp_path);
                        return Ok(());
                    }

                    frame_count += 1;
                    
                    // Check if receiver is still connected periodically
                    if frame_count % 30 == 0 && frame_tx.is_closed() {
                        println!("Video stream receiver closed");
                        let _ = std::fs::remove_file(&temp_path);
                        return Ok(());
                    }
                }
            }
            
            // Early exit if receiver closed
            if frame_tx.is_closed() {
                let _ = std::fs::remove_file(&temp_path);
                return Ok(());
            }
        }

        // Send flush
        let _ = decoder.send_eof();
        let mut decoded = ffmpeg::util::frame::Video::empty();
        while decoder.receive_frame(&mut decoded).is_ok() && !frame_tx.is_closed() {
            let elapsed = last_frame_time.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
            last_frame_time = Instant::now();
            
            let mut rgb_frame = ffmpeg::util::frame::Video::empty();
            if scaler.run(&decoded, &mut rgb_frame).is_ok() {
                let data = rgb_frame.data(0).to_vec();
                let width = rgb_frame.width();
                let height = rgb_frame.height();
                let pts = decoded.pts().unwrap_or(0);
                let timestamp_ms = (pts as f64 * f64::from(time_base) * 1000.0) as i64;

                let video_frame = VideoFrame {
                    data,
                    width,
                    height,
                    timestamp_ms,
                };

                if frame_tx.blocking_send(video_frame).is_err() {
                    break;
                }
                frame_count += 1;
            }
        }

        // Clean up temp file
        let _ = std::fs::remove_file(&temp_path);

        println!("Video playback complete: {} frames in {:.2}s", 
                 frame_count, start_time.elapsed().as_secs_f64());
        Ok(())
    }
}
