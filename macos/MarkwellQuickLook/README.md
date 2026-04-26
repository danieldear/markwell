# Markwell Quick Look Extension (macOS)

This directory contains a working scaffold for a macOS Quick Look preview
extension target that can render Markdown documents in Finder Quick Look.

## Contents

- `Info.plist`: extension metadata, supported content types, principal class.
- `Sources/PreviewProvider.swift`: `QLPreviewProvider` implementation.

## Create The Extension Target In Xcode

1. Open your app packaging project/workspace in Xcode.
2. Add a new target: `App Extension` -> `Quick Look Preview Extension`.
3. Set target name to `MarkwellQuickLook`.
4. Replace generated `Info.plist` and source file with files from this folder.
5. Ensure deployment target is macOS 12.0 or newer.
6. Set extension bundle identifier to:
   - `dev.markwell.desktop.quicklook`
7. Embed the extension in the host app target.

## Test Locally

1. Build and run the host app with the extension embedded.
2. Run:

```bash
qlmanage -r
qlmanage -r cache
```

3. In Finder, select a `.md` file and press `Space`.

## Notes

- This scaffold currently uses a lightweight Markdown-to-HTML fallback in Swift.
- The intended next step is replacing the fallback renderer with `markdown-ffi`
  so Quick Look preview matches Markwell renderer behavior.
