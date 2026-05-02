# MD Star Quick Look Extension (macOS)

A macOS Quick Look preview extension that renders Markdown documents with proper HTML formatting in Finder Quick Look (press `Space` on any `.md` file).

## Contents

- `Info.plist` — extension metadata, supported content types, principal class.
- `Sources/PreviewProvider.swift` — `QLPreviewProvider` + `QLPreviewingController` implementation.
- `Sources/MarkdownRenderer.swift` — pure-Swift Markdown-to-HTML renderer.
- `project.yml` — [xcodegen](https://github.com/yonaskolb/XcodeGen) project spec.
- `build.sh` — build the extension and embed it in the MD Star app bundle.

## Requirements

- macOS 12.0 or later
- Xcode command-line tools
- [xcodegen](https://github.com/yonaskolb/XcodeGen): `brew install xcodegen`

## Build & Install

### Quick start (development)

```bash
cd macos/MarkwellQuickLook

# Build the extension and embed it in the Tauri debug app bundle
./build.sh --app-bundle ../../target/debug/bundle/macos/MD\ Star.app

# Or embed in the release bundle
./build.sh --app-bundle ../../target/release/bundle/macos/MD\ Star.app
```

The script:
1. Generates the Xcode project from `project.yml` (via xcodegen).
2. Builds `MarkwellQuickLook.appex` with ad-hoc signing.
3. Copies the `.appex` into `MD Star.app/Contents/PlugIns/`.
4. Re-signs the app bundle and reloads the Quick Look daemon.

### Manual Xcode workflow

1. `xcodegen generate` — creates `MarkwellQuickLook.xcodeproj`.
2. Open the project in Xcode and build the `MarkwellQuickLook` scheme.
3. Copy the built `.appex` from DerivedData into `MD Star.app/Contents/PlugIns/`.
4. Run `qlmanage -r && qlmanage -r cache`.

## Test

```bash
# Confirm the extension is registered
qlmanage -m | grep -i markwell

# Preview a file from the command line
qlmanage -p /path/to/file.md
```

Then in Finder: select any `.md` file and press `Space`.

## Notes

- The extension must be embedded inside `MD Star.app` for macOS to register it; it cannot be installed as a standalone bundle.
- Ad-hoc signing (`CODE_SIGN_IDENTITY="-"`) works for development. A Developer ID is required for distribution.
- The Swift renderer handles headings, bold/italic, inline code, fenced code blocks, blockquotes, ordered/unordered lists, links, images, and horizontal rules.
