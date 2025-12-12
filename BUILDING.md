# Building Elysia

This guide explains how to build Elysia from source.

## Prerequisites

- [Flutter](https://flutter.dev/docs/get-started/install) 3.38+
- [Rust](https://rustup.rs/) 1.75+
- [flutter_rust_bridge_codegen](https://cjycode.com/flutter_rust_bridge/) 2.11.1
- Linux: `clang cmake ninja-build pkg-config libgtk-3-dev liblzma-dev libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libavdevice-dev libswscale-dev libswresample-dev`

## Installing flutter_rust_bridge_codegen

```bash
cargo install flutter_rust_bridge_codegen@2.11.1
```

## Building

### 1. Generate Flutter Rust Bridge bindings

The Flutter-Rust bindings are auto-generated and not checked into version control. Generate them first:

```bash
cd crates/flutter_frontend
flutter pub get
flutter_rust_bridge_codegen generate
```

This will generate:
- `lib/api.dart` - Dart API functions
- `lib/frb_generated.dart` - Generated Dart glue code
- `lib/frb_generated.io.dart` - Platform-specific code for native
- `lib/frb_generated.web.dart` - Platform-specific code for web
- `../backend/src/frb_generated.rs` - Generated Rust glue code

### 2. Build Rust backend

```bash
cargo build --release -p backend
```

This produces `target/release/libbackend.so` (Linux), `libbackend.dylib` (macOS), or `backend.dll` (Windows).

### 3. Build Flutter app

```bash
cd crates/flutter_frontend
flutter build linux --release
```

The built app will be in `crates/flutter_frontend/build/linux/x64/release/bundle/`.

## Development Workflow

For development, you typically only need to regenerate bindings when you change the Rust FFI API in `crates/backend/src/ffi.rs`.

### Quick rebuild after Rust changes:

```bash
# Rebuild Rust backend
cargo build -p backend

# If you changed FFI signatures, regenerate bindings
cd crates/flutter_frontend
flutter_rust_bridge_codegen generate

# Run Flutter app
flutter run
```

## Troubleshooting

### "frb_generated.rs not found" error

Run `flutter_rust_bridge_codegen generate` in the `crates/flutter_frontend` directory.

### "libbackend.so not found" error

Make sure you've built the Rust backend with `cargo build --release -p backend`.

### FFI type mismatch errors

After changing the Rust FFI API, always regenerate bindings with `flutter_rust_bridge_codegen generate`.

## CI/CD

The GitHub Actions workflow automatically:
1. Installs flutter_rust_bridge_codegen
2. Generates bindings
3. Builds the Rust backend
4. Builds the Flutter app
5. Creates an AppImage

See `.github/workflows/build-linux.yml` for details.
