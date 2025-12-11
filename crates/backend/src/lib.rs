pub mod components;
pub mod game_providers;
pub mod globals;
pub mod runners;
pub mod settings;
pub mod video_decoder;

// FFI module for Flutter integration
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod ffi;

// Re-export FFI functions at crate root for easier access
pub use ffi::*;
