# ISI Research - Build Commands
# Run `just` to see all available commands

set shell := ["bash", "-cu"]

# macOS build environment
export MACOSX_DEPLOYMENT_TARGET := "10.15"

# Show available commands
default:
    @just --list

# ============================================================================
# DESKTOP
# ============================================================================

# Setup desktop app (install deps)
setup-desktop:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "=== Desktop Prerequisites ==="
    if ! command -v cargo >/dev/null 2>&1; then
        echo "cargo not found. Install Rust: https://rustup.rs"
        exit 1
    fi
    echo "✓ cargo $(cargo --version | cut -d' ' -f2)"
    if ! command -v npm >/dev/null 2>&1; then
        echo "npm not found. Install Node.js: https://nodejs.org"
        exit 1
    fi
    echo "✓ npm $(npm --version)"
    if ! command -v cargo-tauri >/dev/null 2>&1; then
        echo "→ Installing tauri-cli..."
        cargo install tauri-cli
    fi
    echo "✓ tauri-cli installed"
    echo ""
    echo "Run: just dev-desktop"

# Install desktop dependencies
deps-desktop:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo fetch
    cd crates/isi-desktop/ui && npm install

# Run desktop app in dev mode
dev-desktop: deps-desktop
    cd crates/isi-desktop && cargo tauri dev

# Build desktop app for macOS
build-desktop: deps-desktop
    #!/usr/bin/env bash
    set -euo pipefail
    (cd crates/isi-desktop/ui && npm run build)
    (cd crates/isi-desktop && cargo tauri build)

# ============================================================================
# WEBSITE
# ============================================================================

# Setup website (install deps)
setup-website:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "=== Website Prerequisites ==="
    if command -v npm >/dev/null 2>&1; then
        echo "✓ npm $(npm --version)"
        echo "Run: just dev-website"
    else
        echo "npm not found. Install Node.js: https://nodejs.org"
        exit 1
    fi

# Install website dependencies
deps-website:
    cd website && npm install

# Run website dev server
dev-website: deps-website
    cd website && npm run dev

# Build website (outputs to website/dist)
build-website: deps-website
    cd website && npm run build

# Preview website build locally
preview-website:
    cd website && npm run preview

# ============================================================================
# ALL
# ============================================================================

# Build everything
build-all: build-desktop build-website

# Run Rust tests
test:
    cargo test --workspace

# Format all code
fmt:
    cargo fmt --all

# Check for Rust issues
check:
    cargo clippy --workspace

# Clean build artifacts
clean:
    cargo clean
    rm -rf crates/isi-desktop/ui/dist
    rm -rf crates/isi-desktop/ui/node_modules
    rm -rf website/dist
    rm -rf website/node_modules
