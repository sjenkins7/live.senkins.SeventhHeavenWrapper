# Seventh Heaven Wrapper

POC using Tauri to wrap Seventh Heven, fully managing a WINE prefix & keep the user informed throughout.

Expect things to be broken / half implemented while things are being proved out.

# Required tooling

* nvm
* rustup
* python (for tooling)

```
nvm use --latest # will be constrained by .nvmrc
rustup toolchain install stable
```

# Updating dependency sources

Any time the app changes its dependencies, these need to be re-run

```
pipx install flatpak-builder-tools/node
flatpak-node-generator --no-requests-cache -r -o node-sources.json yarn tauri-app/yarn.lock

./flatpak-builder-tools/cargo/flatpak-cargo-generator.py -o cargo-sources.json tauri-app/src-tauri/Cargo.lock
```

# Build

Building without a cache will take a while.

```
flatpak-builder --user build --force-clean --install live.senkins.SeventhHeavenWrapper.yml
```