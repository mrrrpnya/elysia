.PHONY: help install-deps gen build build-rust build-flutter run clean test all

# Default target
help:
	@echo "Elysia Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  install-deps    - Install all dependencies (Rust, Flutter, codegen)"
	@echo "  gen             - Generate Flutter Rust Bridge bindings"
	@echo "  build-rust      - Build Rust backend (release)"
	@echo "  build-flutter   - Build Flutter app (release)"
	@echo "  build           - Generate bindings and build everything"
	@echo "  run             - Run the Flutter app in debug mode"
	@echo "  clean           - Clean all build artifacts"
	@echo "  test            - Run all tests"
	@echo "  all             - Complete build from scratch"
	@echo ""

# Install all dependencies
install-deps:
	@echo "==> Installing flutter_rust_bridge_codegen..."
	cargo install flutter_rust_bridge_codegen@2.11.1 --force
	@echo "==> Installing Flutter dependencies..."
	cd crates/flutter_frontend && flutter pub get
	@echo "==> Dependencies installed!"

# Generate Flutter Rust Bridge bindings
gen:
	@echo "==> Generating Flutter Rust Bridge bindings..."
	cd crates/flutter_frontend && flutter_rust_bridge_codegen generate
	@echo "==> Bindings generated!"

# Build Rust backend in release mode
build-rust:
	@echo "==> Building Rust backend (release)..."
	cargo build --release -p backend
	@echo "==> Rust backend built: target/release/libbackend.so"

# Build Flutter app in release mode
build-flutter: gen
	@echo "==> Preparing Rust library for Flutter..."
	@mkdir -p crates/flutter_frontend/linux/libs
	@if [ -f target/release/libbackend.so ]; then \
		cp target/release/libbackend.so crates/flutter_frontend/linux/libs/; \
		echo "==> Rust library copied to Flutter libs"; \
	else \
		echo "ERROR: Rust library not found. Run 'make build-rust' first."; \
		exit 1; \
	fi
	@echo "==> Building Flutter app (release)..."
	cd crates/flutter_frontend && flutter build linux --release
	@echo "==> Flutter app built: crates/flutter_frontend/build/linux/x64/release/bundle/"

# Build everything
build: gen build-rust build-flutter
	@echo "==> Build complete!"

# Run Flutter app in debug mode
run: gen
	@echo "==> Building Rust backend (debug)..."
	@cargo build -p backend
	@echo "==> Running Flutter app..."
	cd crates/flutter_frontend && flutter run

# Clean all build artifacts
clean:
	@echo "==> Cleaning Rust build artifacts..."
	cargo clean
	@echo "==> Cleaning Flutter build artifacts..."
	cd crates/flutter_frontend && flutter clean
	@echo "==> Removing generated files..."
	@rm -f crates/flutter_frontend/lib/api.dart
	@rm -f crates/flutter_frontend/lib/frb_generated*.dart
	@rm -f crates/backend/src/frb_generated.rs
	@rm -rf crates/flutter_frontend/linux/libs
	@echo "==> Clean complete!"

# Run all tests
test:
	@echo "==> Running Rust tests..."
	cargo test --workspace
	@echo "==> Running Flutter tests..."
	cd crates/flutter_frontend && flutter test
	@echo "==> All tests passed!"

# Complete build from scratch
all: clean install-deps build
	@echo "==> Complete build finished!"

# Development helpers
.PHONY: dev-rust dev-flutter

# Watch and rebuild Rust on changes (requires cargo-watch)
dev-rust:
	@echo "==> Watching Rust code for changes..."
	cargo watch -x 'build -p backend'

# Run Flutter in hot-reload mode
dev-flutter: gen
	@echo "==> Running Flutter in hot-reload mode..."
	cd crates/flutter_frontend && flutter run
