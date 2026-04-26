# Release Checklist

Use this checklist before the first public GitHub release.

## Repository

- Set the GitHub remote.
- Set a default `REPO=owner/repository` in `install.sh` after the final repository path is known, or keep requiring the environment variable.
- Verify `./install.sh --link-app` links an installed macOS app binary to `~/.local/bin/md`.
- Confirm `README.md` describes the current Tauri implementation.
- Confirm `docs/index.html` is enabled as the GitHub Pages entry point if Pages is used.
- Keep commit messages clean; do not add `Co-authored-by` trailers.

## Local Verification

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cd crates/markdown-app && cargo tauri build
```

## macOS Verification

- Install the generated DMG.
- Right-click a `.md` file and choose Open With -> Markwell.
- Confirm the app opens the selected file.
- Confirm dragging the custom titlebar moves the window.
- Confirm the app opens normally from Finder.
- Run `./install.sh --link-app` and confirm `md --help` resolves from the linked app binary.

## GitHub Release

```bash
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

The release workflow should create a draft release. Inspect the attached
artifacts before publishing.

## Known Deferred Items

- macOS signing and notarization.
- Quick Look extension embedding/signing from `macos/MarkwellQuickLook`.
- Linux MIME registration installer.
- Windows public support policy.
