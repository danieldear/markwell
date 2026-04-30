#!/usr/bin/env bash
# Master build script for Markwell.
#
# Builds the Tauri app (Rust + frontend) and, on macOS, compiles and embeds
# the Quick Look preview extension into the resulting app bundle.
#
# Usage:
#   ./build.sh [options]
#
# Options:
#   --debug              Build in debug mode (default: release)
#   --release            Build in release mode (default)
#   --target <triple>    Rust target triple (e.g. universal-apple-darwin)
#   --skip-quicklook     Skip Quick Look extension build on macOS
#   --help               Show this message

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
APP_DIR="$REPO_ROOT/crates/markdown-app"
QL_DIR="$REPO_ROOT/macos/MarkwellQuickLook"

# ── Defaults ─────────────────────────────────────────────────────────────────
PROFILE="release"
TARGET=""
SKIP_QL=false

# ── Helpers ───────────────────────────────────────────────────────────────────
error()   { printf '\033[31merror\033[0m: %s\n' "$1" >&2; exit 1; }
info()    { printf '\033[34m  →\033[0m %s\n' "$1"; }
ok()      { printf '\033[32m  ✓\033[0m %s\n' "$1"; }
section() { printf '\n\033[1m%s\033[0m\n' "$1"; }

# ── Argument parsing ──────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug)          PROFILE="debug";  shift ;;
    --release)        PROFILE="release"; shift ;;
    --target)         TARGET="$2";      shift 2 ;;
    --skip-quicklook) SKIP_QL=true;     shift ;;
    --help)
      sed -n '/^# Usage/,/^[^#]/p' "$0" | grep '^#' | sed 's/^# \{0,1\}//'
      exit 0 ;;
    *) error "Unknown option: $1" ;;
  esac
done

IS_MACOS=false
[[ "$(uname -s)" == "Darwin" ]] && IS_MACOS=true

# ── Step 1: Build Tauri app ───────────────────────────────────────────────────
section "Building Markwell app"

TAURI_ARGS=()
[[ "$PROFILE" == "debug" ]] && TAURI_ARGS+=(--debug)
[[ -n "$TARGET" ]] && TAURI_ARGS+=(--target "$TARGET")

info "cargo tauri build ${TAURI_ARGS[*]:-}"
(cd "$APP_DIR" && cargo tauri build "${TAURI_ARGS[@]+"${TAURI_ARGS[@]}"}")
ok "Tauri build complete"

# ── Step 2: Locate the built .app bundle ─────────────────────────────────────
# Tauri puts the bundle under target/<target>/release/bundle/macos/ or
# target/release/bundle/macos/ when no --target is given.
if [[ -n "$TARGET" ]]; then
  BUNDLE_BASE="$REPO_ROOT/target/$TARGET/$PROFILE/bundle/macos"
else
  BUNDLE_BASE="$REPO_ROOT/target/$PROFILE/bundle/macos"
fi

APP_BUNDLE="$(find "$BUNDLE_BASE" -maxdepth 1 -name "*.app" | head -1)"
[[ -d "$APP_BUNDLE" ]] || error "Could not find .app bundle under $BUNDLE_BASE"
ok "Found app bundle: $APP_BUNDLE"

# ── Step 3: Quick Look extension (macOS only) ─────────────────────────────────
if $IS_MACOS && ! $SKIP_QL; then
  section "Building Quick Look extension"

  # NOTE: macOS 26+ no longer loads generators from ~/Library/QuickLook/.
  # The generator is compiled and installed there, but Finder will not invoke
  # it unless the bundle is signed with a Developer ID Application certificate
  # and notarized. Use --skip-quicklook to suppress this step, or install
  # QLMarkdown (brew install --cask qlmarkdown) as a signed alternative.
  command -v clang &>/dev/null || error "clang not found -- install Xcode command-line tools"

  "$QL_DIR/build.sh" --app-bundle "$APP_BUNDLE"
  info "NOTE: Quick Look preview requires a Developer ID certificate on macOS 26+."
  info "      Install QLMarkdown as a signed alternative: brew install --cask qlmarkdown"
elif $SKIP_QL; then
  info "Skipping Quick Look extension (--skip-quicklook)"
elif ! $IS_MACOS; then
  info "Skipping Quick Look extension (not macOS)"
fi

# ── Done ──────────────────────────────────────────────────────────────────────
section "Done"
ok "App bundle: $APP_BUNDLE"
