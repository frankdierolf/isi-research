# ISI Research - Build Commands
set shell := ["bash", "-cu"]
export MACOSX_DEPLOYMENT_TARGET := "10.15"

# Show available commands
default:
    @just --list --unsorted

# ============================================================================
# DESKTOP
# ============================================================================

# Run desktop app in dev mode
[group('desktop')]
dev: _deps
    cd crates/isi-desktop && cargo tauri dev

# Build desktop app for release
[group('desktop')]
build: _deps
    (cd crates/isi-desktop/ui && npm run build)
    (cd crates/isi-desktop && cargo tauri build)

# Install dependencies
[private]
_deps:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo fetch
    cd crates/isi-desktop/ui && npm install

# ============================================================================
# WEBSITE
# ============================================================================

# Run website dev server
[group('website')]
dev-web:
    cd website && npm install && npm run dev

# Build website
[group('website')]
build-web:
    cd website && npm install && npm run build

# ============================================================================
# ALL
# ============================================================================

# Format all code
[group('all')]
fmt:
    cargo fmt --all

# Check for issues
[group('all')]
check:
    cargo clippy --workspace

# Clean build artifacts
[group('all')]
clean:
    cargo clean
    rm -rf crates/isi-desktop/ui/dist website/dist
