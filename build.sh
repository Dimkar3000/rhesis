#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

BUILD_DIR="$SCRIPT_DIR/build"
VENV_DIR="$BUILD_DIR/venv"
GENERATOR_SCRIPT="$BUILD_DIR/flatpak-cargo-generator.py"
CARGO_SOURCES="$BUILD_DIR/cargo-sources.json"
FLATPAK_OUTPUT="$BUILD_DIR/flatpak-build-dir"
FLATPAK_STATE="$BUILD_DIR/flatpak-state"

mkdir -p "$BUILD_DIR"

# --- Python virtual environment ---
if [ ! -d "$VENV_DIR" ]; then
    echo "Creating Python virtual environment..."
    python3 -m venv "$VENV_DIR"
fi

source "$VENV_DIR/bin/activate"

echo "Installing Python dependencies..."
pip install -q -r requirements.txt

# --- Download flatpak-cargo-generator and generate vendored sources ---
GENERATOR_URL="https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py"

if [ ! -f "$GENERATOR_SCRIPT" ]; then
    echo "Downloading flatpak-cargo-generator.py..."
    curl -fsSLo "$GENERATOR_SCRIPT" "$GENERATOR_URL"
fi

echo "Generating cargo-sources.json..."
python "$GENERATOR_SCRIPT" Cargo.lock -o "$CARGO_SOURCES"

# --- Install flatpak runtimes and extensions ---
echo "Ensuring flatpak runtimes and extensions are installed..."
flatpak install --user --noninteractive flathub org.kde.Platform//6.9
flatpak install --user --noninteractive flathub org.kde.Sdk//6.9
flatpak install --user --noninteractive flathub org.freedesktop.Sdk.Extension.rust-stable//24.08
flatpak install --user --noninteractive flathub org.freedesktop.Sdk.Extension.openjdk17//24.08

# --- Build flatpak ---
echo "Building flatpak..."
flatpak-builder \
    --user \
    --force-clean \
    --state-dir "$FLATPAK_STATE" \
    "$FLATPAK_OUTPUT" \
    org.dimkar.rhesis.json

echo "Build complete."

echo "Exporting to local repo..."
flatpak build-export rhesis-master "$FLATPAK_OUTPUT"
echo "Creating bundle..."
flatpak build-bundle rhesis-master rhesis.flatpak org.dimkar.rhesis
echo "Bundle created: rhesis.flatpak"
