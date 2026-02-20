#!/usr/bin/env bash
set -euo pipefail

REPO="uinaf/tccutil"
BINARY_NAME="tccutil-rs"
INSTALL_PATH="/usr/local/bin/${BINARY_NAME}"

usage() {
  cat <<'USAGE'
Install tccutil-rs from GitHub Releases.

Usage:
  scripts/install.sh [VERSION]

Examples:
  scripts/install.sh          # install latest release
  scripts/install.sh v0.1.1   # install a specific release

Notes:
  - macOS only
  - installs to /usr/local/bin/tccutil-rs
USAGE
}

error() {
  printf 'error: %s\n' "$1" >&2
  exit 1
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ "$(uname -s)" != "Darwin" ]]; then
  error "tccutil-rs is macOS-only"
fi

arch="$(uname -m)"
case "$arch" in
  arm64) platform="darwin-arm64" ;;
  x86_64) platform="darwin-amd64" ;;
  *) error "unsupported architecture: $arch" ;;
esac

if ! command -v curl >/dev/null 2>&1; then
  error "curl is required"
fi
if ! command -v shasum >/dev/null 2>&1; then
  error "shasum is required"
fi
if ! command -v tar >/dev/null 2>&1; then
  error "tar is required"
fi

version_arg="${1:-}"
if [[ -z "$version_arg" ]]; then
  version="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -n1)"
  [[ -n "$version" ]] || error "failed to resolve latest release version"
else
  if [[ "$version_arg" == v* ]]; then
    version="$version_arg"
  else
    version="v${version_arg}"
  fi
fi

asset="${BINARY_NAME}_${version}_${platform}.tar.gz"
base_url="https://github.com/${REPO}/releases/download/${version}"
asset_url="${base_url}/${asset}"
checksums_url="${base_url}/checksums.txt"

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

printf 'Installing %s (%s) from %s\n' "$BINARY_NAME" "$platform" "$version"

curl -fsSL "$asset_url" -o "${tmp_dir}/${asset}"
curl -fsSL "$checksums_url" -o "${tmp_dir}/checksums.txt"

expected_sha="$(grep "  ${asset}$" "${tmp_dir}/checksums.txt" | awk '{print $1}')"
[[ -n "$expected_sha" ]] || error "checksum not found for ${asset}"

actual_sha="$(shasum -a 256 "${tmp_dir}/${asset}" | awk '{print $1}')"
if [[ "$expected_sha" != "$actual_sha" ]]; then
  error "sha256 verification failed for ${asset}"
fi

tar -xzf "${tmp_dir}/${asset}" -C "$tmp_dir"
[[ -f "${tmp_dir}/${BINARY_NAME}" ]] || error "archive missing ${BINARY_NAME}"

if [[ -w "$(dirname "$INSTALL_PATH")" ]]; then
  install -m 0755 "${tmp_dir}/${BINARY_NAME}" "$INSTALL_PATH"
else
  sudo install -m 0755 "${tmp_dir}/${BINARY_NAME}" "$INSTALL_PATH"
fi

printf 'Installed: %s\n' "$INSTALL_PATH"
"$INSTALL_PATH" --version
