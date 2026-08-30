#!/usr/bin/env bash
#
# package_linux.sh — Package Yuhina Linux release artifacts.
#
# Produces:
#   build/dist/yuhina-{VERSION}-linux-x64.tar.gz      (portable)
#   build/dist/yuhina-{VERSION}-linux-x64.AppImage    (AppImage)
#
# Usage:
#   bash build/linux/package_linux.sh <VERSION>
#
# Prerequisites:
#   - flutter build linux --release already run
#   - AppImage tooling: linuxdeploy + appimagetool (auto-downloaded to build/dist/tools if missing)
#
# Env overrides:
#   LINUXDEPLOY   path to linuxdeploy AppImage (default: download)
#   APPIMAGETOOL  path to appimagetool AppImage (default: download)
#   NO_FUSE=1     force --appimage-extract-and-run (runners without FUSE)
#
set -euo pipefail

VERSION="${1:?usage: package_linux.sh <VERSION>}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FLUTTER_ROOT="$ROOT/yuhina"
DIST="$ROOT/build/dist"
TOOLS="$DIST/tools"
ARCH="x64"
OS="linux"
BUNDLE_DIR="$FLUTTER_ROOT/build/linux/x64/release/bundle"

if [ ! -x "$BUNDLE_DIR/yuhina" ]; then
  echo "::error::Linux bundle not found at $BUNDLE_DIR. Run 'flutter build linux --release' first."
  exit 1
fi

mkdir -p "$TOOLS"
BASE_NAME="yuhina-${VERSION}-${OS}-${ARCH}"

fetch_tool() {
  local url="$1" dest="$2"
  if [ ! -f "$dest" ]; then
    echo ">> downloading $(basename "$dest")"
    local attempt=0 ok=0
    for attempt in 1 2 3 4 5; do
      if curl -L --fail --retry 5 --retry-all-errors --retry-delay 3 -o "$dest" "$url"; then
        ok=1
        break
      fi
      echo ">> attempt $attempt failed; retrying in 5s..."
      sleep 5
      rm -f "$dest"
    done
    if [ "$ok" != "1" ] || [ ! -s "$dest" ]; then
      echo "::error::failed to download $url" >&2
      exit 1
    fi
  fi
  chmod +x "$dest"
}

run_appimage() {
  local tool="$1"; shift
  if [ "${NO_FUSE:-0}" = "1" ]; then
    "$tool" --appimage-extract-and-run "$@"
  else
    "$tool" "$@"
  fi
}

# ---------------------------------------------------------------------------
# 1. Portable tar.gz
# ---------------------------------------------------------------------------
echo ">> Packaging portable tar.gz"
STAGE="$DIST/$BASE_NAME"
rm -rf "$STAGE"
mkdir -p "$STAGE"
cp -r "$BUNDLE_DIR/." "$STAGE/"
printf 'yuhina-%s\n' "$VERSION" > "$STAGE/VERSION"

TARBALL="$DIST/$BASE_NAME.tar.gz"
tar -C "$STAGE" -czf "$TARBALL" .
rm -rf "$STAGE"
echo ">> $TARBALL ($(du -h "$TARBALL" | cut -f1))"

# ---------------------------------------------------------------------------
# 2. AppImage (best-effort: if tooling cannot be fetched, the portable
#    tar.gz above is still a valid release artifact)
# ---------------------------------------------------------------------------
echo ">> Building AppImage"
LINUXDEPLOY="${LINUXDEPLOY:-$TOOLS/linuxdeploy-x86_64.AppImage}"
APPIMAGETOOL="${APPIMAGETOOL:-$TOOLS/appimagetool-x86_64.AppImage}"

fetch_optional() {
  local url="$1" dest="$2" name="$3"
  if [ -f "$dest" ]; then return 0; fi
  echo ">> downloading $name"
  local attempt=0 ok=0
  for attempt in 1 2 3; do
    if curl -L --fail --retry 3 --retry-all-errors -o "$dest" "$url"; then
      ok=1; break
    fi
    echo ">> attempt $attempt failed; retrying in 5s..."
    sleep 5; rm -f "$dest"
  done
  [ "$ok" = "1" ] && [ -s "$dest" ] && chmod +x "$dest"
}

if ! fetch_optional \
    "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage" \
    "$LINUXDEPLOY" "linuxdeploy"; then
  echo "::warning::linuxdeploy download failed; skipping AppImage (tar.gz still shipped)"
  echo ">> Done (AppImage skipped)."
  ls -lh "$TARBALL"
  exit 0
fi
if ! fetch_optional \
    "https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage" \
    "$APPIMAGETOOL" "appimagetool"; then
  echo "::warning::appimagetool download failed; skipping AppImage (tar.gz still shipped)"
  echo ">> Done (AppImage skipped)."
  ls -lh "$TARBALL"
  exit 0
fi
export PATH="$TOOLS:$PATH"

APPDIR="$DIST/AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
cp -r "$BUNDLE_DIR/." "$APPDIR/usr/bin/"

# desktop entry
mkdir -p "$APPDIR/usr/share/applications"
cat > "$APPDIR/usr/share/applications/yuhina.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Yuhina
GenericName=Minecraft Launcher
Comment=Modern cross-platform Minecraft launcher
Exec=yuhina
Icon=yuhina
Terminal=false
Categories=Game;Utility;
StartupWMClass=yuhina
EOF

# locate an app icon (first match wins); fall back to a generated placeholder
ICON_SRC=""
for cand in \
  "$FLUTTER_ROOT/assets/icon.png" \
  "$FLUTTER_ROOT/assets/images/icon.png" \
  "$STAGE/data/flutter_assets/assets/icon.png" \
  "$STAGE/data/flutter_assets/icon.png"; do
  if [ -f "$cand" ]; then ICON_SRC="$cand"; break; fi
done
if [ -z "$ICON_SRC" ]; then
  echo ">> No icon found; generating placeholder icon.png"
  ICON_SRC="$DIST/icon.png"
  python3 - "$ICON_SRC" <<'PYEOF'
import struct, sys, zlib

def chunk(tag, data):
    c = tag + data
    return struct.pack(">I", len(data)) + c + struct.pack(">I", zlib.crc32(c) & 0xFFFFFFFF)

w = h = 256
raw = b""
for y in range(h):
    raw += b"\x00" + b"\x2e\x8b\x57\xff" * w  # opaque brand green
png = (b"\x89PNG\r\n\x1a\n"
       + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
       + chunk(b"IDAT", zlib.compress(raw, 9))
       + chunk(b"IEND", b""))
open(sys.argv[1], "wb").write(png)
PYEOF
fi
mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"
cp "$ICON_SRC" "$APPDIR/usr/share/icons/hicolor/256x256/apps/yuhina.png"

run_appimage "$LINUXDEPLOY" \
  --appdir "$APPDIR" \
  --desktop-file "$APPDIR/usr/share/applications/yuhina.desktop" \
  --icon-file "$ICON_SRC"

APPIMAGE="$DIST/$BASE_NAME.AppImage"
run_appimage "$APPIMAGETOOL" "$APPDIR" "$APPIMAGE"
rm -rf "$APPDIR"
echo ">> $APPIMAGE ($(du -h "$APPIMAGE" | cut -f1))"

echo ">> Done."
ls -lh "$TARBALL" "$APPIMAGE"