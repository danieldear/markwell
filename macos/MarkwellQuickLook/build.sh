#!/usr/bin/env bash
# Build, embed, and register the MarkwellQuickLook extension.
#
# This script builds the Quick Look app extension target with xcodebuild,
# embeds the .appex into a MD Star.app bundle, re-signs for local use, and
# refreshes the Quick Look daemon.
#
# Usage:
#   ./build.sh [--app-bundle /path/to/MD Star.app] [--debug|--release]
#   ./build.sh /path/to/MD Star.app [--debug|--release]
#
# Defaults:
#   app bundle: /Applications/MD Star.app
#   config:     Release

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/.build"
BUNDLE_NAME="MarkwellQuickLook.appex"
HOST_APP="/Applications/MD Star.app"
CONFIGURATION="Release"
EXT_ENTITLEMENTS="$SCRIPT_DIR/MarkwellQuickLook.entitlements"

error() { printf '\033[31merror\033[0m: %s\n' "$1" >&2; exit 1; }
info()  { printf '\033[34m  ->\033[0m %s\n' "$1"; }
ok()    { printf '\033[32m  ok\033[0m %s\n' "$1"; }

while [[ $# -gt 0 ]]; do
  case "$1" in
    --app-bundle) HOST_APP="$2"; shift 2 ;;
    --debug) CONFIGURATION="Debug"; shift ;;
    --release) CONFIGURATION="Release"; shift ;;
    *.app) HOST_APP="$1"; shift ;;
    *) error "Unknown option: $1" ;;
  esac
done

command -v xcodebuild &>/dev/null || error "xcodebuild not found -- install Xcode command-line tools"
command -v xcodegen &>/dev/null || error "xcodegen not found -- install with: brew install xcodegen"
[[ -d "$HOST_APP" ]] || error "MD Star.app not found at $HOST_APP -- install the app first"
PLUGINS_DIR="$HOST_APP/Contents/PlugIns"
DERIVED_DATA="$BUILD_DIR/DerivedData"
APP_EX_PATH="$DERIVED_DATA/Build/Products/$CONFIGURATION/$BUNDLE_NAME"

cd "$SCRIPT_DIR"
mkdir -p "$BUILD_DIR"

info "Generating Xcode project"
xcodegen generate --spec "$SCRIPT_DIR/project.yml"

info "Building Quick Look extension ($CONFIGURATION)"
xcodebuild \
  -project "$SCRIPT_DIR/MarkwellQuickLook.xcodeproj" \
  -scheme MarkwellQuickLook \
  -configuration "$CONFIGURATION" \
  -derivedDataPath "$DERIVED_DATA" \
  CODE_SIGN_IDENTITY="-" \
  CODE_SIGNING_ALLOWED=YES \
  CODE_SIGNING_REQUIRED=NO \
  build >/dev/null

[[ -d "$APP_EX_PATH" ]] || error "Built extension not found at $APP_EX_PATH"
ok "Built $BUNDLE_NAME"

info "Embedding extension in host app"
mkdir -p "$PLUGINS_DIR"
rm -rf "$PLUGINS_DIR/$BUNDLE_NAME"
cp -R "$APP_EX_PATH" "$PLUGINS_DIR/"
ok "Embedded"

# Preserve extension entitlements. Re-sign nested code first, then app bundle.
info "Re-signing extension + host app for local execution"
codesign --force --sign - --entitlements "$EXT_ENTITLEMENTS" "$PLUGINS_DIR/$BUNDLE_NAME"
codesign --force --sign - "$HOST_APP"
ok "Re-signed host app"

info "Refreshing Quick Look registration"
qlmanage -r >/dev/null 2>&1 || true
qlmanage -r cache >/dev/null 2>&1 || true
pluginkit -a "$PLUGINS_DIR/$BUNDLE_NAME" >/dev/null 2>&1 || true

ok "Quick Look refresh complete"
EXT_BUNDLE_ID=$(/usr/libexec/PlistBuddy -c "Print :CFBundleIdentifier" "$PLUGINS_DIR/$BUNDLE_NAME/Contents/Info.plist" 2>/dev/null || true)
if [[ -n "${EXT_BUNDLE_ID:-}" ]]; then
  info "Extension registration status ($EXT_BUNDLE_ID)"
  pluginkit -m -A -D -i "$EXT_BUNDLE_ID" || true
fi
info "Test: select a .md file in Finder and press Space"
