#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
SCRIPT="$ROOT/scripts/build-rootfs.sh"
tmp="$(mktemp -d)"
safe_rootfs="/tmp/nila-rootfs-guard-${tmp##*/}"
rootfs_created=0
cleanup() {
  rm -rf -- "$tmp"
  if [[ "$rootfs_created" == 1 ]]; then rmdir -- "$safe_rootfs"; fi
}
trap cleanup EXIT
if [[ -e "$safe_rootfs" || -L "$safe_rootfs" ]]; then
  echo "ERROR: guard-test path unexpectedly exists: $safe_rootfs" >&2
  exit 1
fi

bash -n "$SCRIPT"
bash "$SCRIPT" --help >/dev/null
if ROOTFS_DIR=relative/path bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: relative output path was accepted" >&2; exit 1
fi
if ROOTFS_DIR="$ROOT/out/test-rootfs" bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: repository-local output path was accepted" >&2; exit 1
fi
if ROOTFS_DIR="/tmp/../etc/nila-rootfs-test" bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: traversal output path was accepted" >&2; exit 1
fi
if ROOTFS_DIR="/etc/nila-rootfs-test" bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: system-directory output path was accepted" >&2; exit 1
fi
ln -s "$tmp" "$tmp/parent-link"
if ROOTFS_DIR="$tmp/parent-link/rootfs" bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: symlinked-parent output path was accepted" >&2; exit 1
fi
if MIRROR=http://example.invalid ROOTFS_DIR="$safe_rootfs" bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: non-HTTPS mirror was accepted" >&2; exit 1
fi
mkdir -- "$safe_rootfs"
rootfs_created=1
if ROOTFS_DIR="$safe_rootfs" bash "$SCRIPT" >/dev/null 2>&1; then
  echo "ERROR: existing output directory was accepted" >&2; exit 1
fi
echo "Rootfs helper guard tests passed."
