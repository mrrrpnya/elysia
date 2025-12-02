// Video decoder module for streaming video frames
// Decodes video URLs frame-by-frame and sends them to Flutter

use anyhow::{Context, Result};
use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::mpsc;

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
}

impl VideoDecoder {
    /// Create a new video decoder
    pub fn new(url: String, frame_tx: mpsc::Sender<VideoFrame>) -> Self {
        Self { url, frame_tx }
    }

    /// Start decoding video and streaming frames
    pub async fn start(self) -> Result<()> {
        // Download video to temporary location
        let video_data = Self::download_video(&self.url).await?;
        
        // Decode frames in a blocking task since ffmpeg operations are CPU-bound
        let frame_tx = self.frame_tx;
        tokio::task::spawn_blocking(move || {
            if let Err(e) = Self::decode_frames(&video_data, frame_tx) {
                eprintln!("Video decoding error: {}", e);
            }
        });

        Ok(())
    }

    /// Download video from URL
    async fn download_video(url: &str) -> Result<Vec<u8>> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .send()
            .await
            .context("Failed to download video")?;

        let bytes = response
            .bytes()
            .await
            .context("Failed to read video bytes")?;

        Ok(bytes.to_vec())
    }

    /// Decode video frames using ffmpeg
    fn decode_frames(video_data: &[u8], frame_tx: mpsc::Sender<VideoFrame>) -> Result<()> {
        use ffmpeg_next as ffmpeg;

        // Initialize ffmpeg
        ffmpeg::init().context("Failed to initialize ffmpeg")?;

        // Create a temporary file for ffmpeg to read from
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("elysia_video_{}.webm", std::process::id()));
        std::fs::write(&temp_path, video_data).context("Failed to write temp video file")?;

        // Open input file
        let mut ictx = ffmpeg::format::input(&temp_path)
            .context("Failed to open video file")?;

        // Find video stream
        let input = ictx
            .streams()
            .best(ffmpeg::media::Type::Video)
            .context("Could not find video stream")?;
        let video_stream_index = input.index();

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

        // Decode frames
        let mut frame_count = 0;
        let time_base = input.time_base();
        
        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                
                let mut decoded = ffmpeg::util::frame::Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = ffmpeg::util::frame::Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;

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

                    // Send frame (non-blocking)
                    if frame_tx.blocking_send(video_frame).is_err() {
                        // Receiver dropped, stop decoding
                        break;
                    }

                    frame_count += 1;
                }
            }
        }

        // Send flush
        decoder.send_eof()?;
        let mut decoded = ffmpeg::util::frame::Video::empty();
        while decoder.receive_frame(&mut decoded).is_ok() {
            let mut rgb_frame = ffmpeg::util::frame::Video::empty();
            scaler.run(&decoded, &mut rgb_frame)?;

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

        // Clean up temp file
        let _ = std::fs::remove_file(&temp_path);

        println!("Decoded {} frames", frame_count);
        Ok(())
    }
}
