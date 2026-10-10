#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
SCRIPT="$ROOT/scripts/build-rootfs.sh"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

bash -n "$SCRIPT"
bash "$SCRIPT" --help >/dev/null
if ROOTFS_DIR=relative/path bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: relative output path was accepted" >&2; exit 1
fi
if ROOTFS_DIR="$ROOT/out/test-rootfs" bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: repository-local output path was accepted" >&2; exit 1
fi
if MIRROR=http://example.invalid ROOTFS_DIR="$tmp/rootfs" bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: non-HTTPS mirror was accepted" >&2; exit 1
fi
mkdir "$tmp/existing"
if ROOTFS_DIR="$tmp/existing" bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: existing output directory was accepted" >&2; exit 1
fi
echo "Rootfs helper guard tests passed."
