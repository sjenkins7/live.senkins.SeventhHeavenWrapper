#!/bin/bash

set -euo pipefail

./flatpak-builder-tools/cargo/flatpak-cargo-generator.py -o cargo-sources.json tauri-app/src-tauri/Cargo.lock
flatpak-node-generator --no-requests-cache -r -o node-sources.json yarn tauri-app/yarn.lock
