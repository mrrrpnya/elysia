use bytes::Bytes;
use freya::prelude::*;
use libwebp::WebPDecodeRGBA;
use reqwest::{Url, header::CONTENT_TYPE};

#[derive(Props, Clone, PartialEq)]
pub struct MyNetworkImageProps {
    /// Width of the image container. Default to `auto`.
    #[props(default = "auto".into())]
    pub width: String,
    /// Height of the image container. Default to `auto`.
    #[props(default = "auto".into())]
    pub height: String,
    /// Min width of the image container.
    pub min_width: Option<String>,
    /// Min height of the image container.
    pub min_height: Option<String>,
    /// URL of the image.
    pub url: ReadOnlySignal<Url>,
    /// Fallback element.
    pub fallback: Option<Element>,
    /// Loading element.
    pub loading: Option<Element>,
    /// Information about the image.
    pub alt: Option<String>,
    /// Aspect ratio of the image.
    pub aspect_ratio: Option<String>,
    /// Cover of the image.
    pub cover: Option<String>,
    /// Image sampling algorithm.
    pub sampling: Option<String>,
}

const CACHE_DIR: &str = "./cache";

#[component]
pub fn MyNetworkImage(
    MyNetworkImageProps {
        width,
        height,
        min_width,
        min_height,
        url,
        fallback,
        loading,
        alt,
        aspect_ratio,
        cover,
        sampling,
    }: MyNetworkImageProps,
) -> Element {
    let focus = use_focus();
    let mut status = use_signal(|| ImageState::Loading);
    let mut assets_tasks = use_signal::<Vec<Task>>(Vec::new);

    let a11y_id = focus.attribute();
    let key = url.to_string();
    let url = url.read();

    if let Ok(asset) = cacache::read_sync(CACHE_DIR, &key) {
        // Image loaded from cache
        status.set(ImageState::Loaded(asset.into()));
    } else {
        to_owned![url];
        use_effect(move || {
            // Cancel previous asset fetching requests
            for asset_task in assets_tasks.write().drain(..) {
                asset_task.cancel();
            }

            // Loading image
            to_owned![key, url];
            let asset_task = spawn(async move {
                let asset = fetch_image(url).await;
                if let Ok(asset_bytes) = asset {
                    let _ = cacache::write_sync(CACHE_DIR, &key, &asset_bytes);

                    // Image loaded
                    status.set(ImageState::Loaded(asset_bytes));
                } else if let Err(_err) = asset {
                    // Image errored
                    status.set(ImageState::Errored);
                }
            });

            assets_tasks.write().push(asset_task);
        });
    }

    match &*status.read_unchecked() {
        ImageState::Loaded(bytes) => {
            let image_data = dynamic_bytes(bytes.clone());
            rsx! {
                image {
                    height,
                    width,
                    min_width,
                    min_height,
                    a11y_id,
                    image_data,
                    a11y_role: "image",
                    a11y_name: alt,
                    aspect_ratio,
                    cover,
                    cache_key: "{url}",
                    sampling,
                }
            }
        }
        ImageState::Loading => {
            if let Some(loading_element) = loading {
                rsx! {{ loading_element }}
            } else {
                rsx! {
                    rect {
                        height,
                        width,
                        min_width,
                        min_height,
                        main_align: "center",
                        cross_align: "center",
                        Loader {}
                    }
                }
            }
        }
        _ => {
            if let Some(fallback_element) = fallback {
                rsx! {{ fallback_element }}
            } else {
                rsx! {
                    rect {
                        height,
                        width,
                        min_width,
                        min_height,
                        main_align: "center",
                        cross_align: "center",
                        label {
                            text_align: "center",
                            "Error"
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
async fn fetch_image(url: Url) -> Result<Bytes, String> {
    let res = reqwest::get(url.clone())
        .await
        .map_err(|e| format!("Failed to fetch image: {e}"))?;

    let content_type = res
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("")
        .to_owned();

    let bytes = res
        .bytes()
        .await
        .map_err(|e| format!("Failed to fetch image: {e}"))?;

    match content_type.as_str() {
        "image/webp" => {
            let (width, height, buf) =
                WebPDecodeRGBA(&bytes).map_err(|e| format!("Failed to decode WebP image: {e}"))?;

            let encoded_bytes = lodepng::encode_memory(
                &buf,
                width as usize,
                height as usize,
                lodepng::ColorType::RGBA,
                8,
            )
            .map_err(|e| format!("Failed to encode PNG image: {e}"))?;

            Ok(encoded_bytes.into())
        }
        _ => Ok(bytes),
    }
}

enum ImageState {
    Loading,
    Loaded(Bytes),
    Errored,
}
