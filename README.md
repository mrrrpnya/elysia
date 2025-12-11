# Elysia

A game launcher with a Flutter frontend and Rust backend.

## Architecture

- **Flutter Frontend** (`crates/flutter_frontend/`): Cross-platform UI built with Flutter
- **Rust Backend** (`crates/backend/`): Core game management logic with integrated FFI bridge
- **Common** (`crates/common/`): Shared utilities

## Supported Game Providers

- HoYoPlay (Genshin Impact, Honkai: Star Rail, Zenless Zone Zero)
- Endfield

## Building

### Prerequisites

- [Flutter](https://flutter.dev/docs/get-started/install) 3.38+
- [Rust](https://rustup.rs/) 1.75+
- Linux: `clang cmake ninja-build pkg-config libgtk-3-dev liblzma-dev`

### Build Commands

```bash
# Get Flutter dependencies
cd crates/flutter_frontend
flutter pub get

# Build for Linux
flutter build linux --release

# Build Rust components
cargo build --release
```

## Development

### Project Structure

```
elysia/
├── crates/
│   ├── flutter_frontend/    # Flutter UI
│   │   ├── lib/
│   │   │   ├── main.dart
│   │   │   └── src/
│   │   │       ├── models/     # Data models
│   │   │       ├── pages/      # Page widgets
│   │   │       ├── providers/  # State management
│   │   │       ├── theme/      # App theme
│   │   │       └── widgets/    # Reusable widgets
│   │   └── pubspec.yaml
│   ├── backend/             # Core Rust logic with FFI
│   │   └── src/
│   │       ├── ffi.rs       # Flutter FFI bridge
│   │       └── ...
│   └── common/              # Shared utilities
└── Cargo.toml
```

### Running Tests

```bash
# Flutter tests
cd crates/flutter_frontend
flutter test

# Rust tests
cargo test --workspace
```

## License

See LICENSE file for details.
