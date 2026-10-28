#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
ROOTFS_DIR="${ROOTFS_DIR:-$ROOT/out/native-rootfs}"
OUTPUT_IMAGE="${OUTPUT_IMAGE:-$ROOT/out/nila-native-rootfs.cpio.gz}"

usage() {
  cat <<'EOF'
Usage:
  ROOTFS_DIR=out/native-rootfs OUTPUT_IMAGE=out/nila-native-rootfs.cpio.gz ./scripts/package-native-rootfs.sh

Packages a marked Nila early-userspace staging tree as a gzip-compressed newc initramfs.
Requires cpio, gzip, and file. The output is a development artifact, not a complete
phone OS image and not proof of device bootability.
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then usage; exit 0; fi
for tool in cpio gzip file; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "ERROR: required tool '$tool' was not found." >&2
    exit 2
  fi
done

if [[ ! -d "$ROOTFS_DIR" || ! -f "$ROOTFS_DIR/.nila-rootfs-staging" ]]; then
  echo "ERROR: ROOTFS_DIR must be an existing Nila staging tree with its marker." >&2
  exit 2
fi
ROOTFS_DIR="$(cd -- "$ROOTFS_DIR" && pwd -P)"
if [[ ! -x "$ROOTFS_DIR/init" || ! -x "$ROOTFS_DIR/bin/busybox" ]]; then
  echo "ERROR: staging tree must contain executable init and bin/busybox." >&2
  exit 2
fi

description="$(file -Lb "$ROOTFS_DIR/bin/busybox")"
case "$description" in
  *ARM*aarch64*|*ARM64*|*aarch64*) ;;
  *) echo "ERROR: BusyBox is not identified as ARM64: $description" >&2; exit 2 ;;
esac
case "$description" in
  *"statically linked"*) ;;
  *) echo "ERROR: BusyBox must be statically linked: $description" >&2; exit 2 ;;
esac

mkdir -p "$(dirname -- "$OUTPUT_IMAGE")"
OUTPUT_IMAGE="$(cd -- "$(dirname -- "$OUTPUT_IMAGE")" && pwd -P)/$(basename -- "$OUTPUT_IMAGE")"
case "$OUTPUT_IMAGE" in
  "$ROOTFS_DIR"/*)
    echo "ERROR: output image must be outside ROOTFS_DIR." >&2
    exit 2
    ;;
esac

temporary="$(mktemp "${OUTPUT_IMAGE}.tmp.XXXXXX")"
trap 'rm -f -- "$temporary"' EXIT
(
  cd -- "$ROOTFS_DIR"
  find . -print0 | cpio --null --owner=0:0 -o --format=newc 2>/dev/null
) | gzip -9 > "$temporary"
gzip -t "$temporary"
mv -f -- "$temporary" "$OUTPUT_IMAGE"
trap - EXIT

printf 'Native Linux initramfs archive created: %s\n' "$OUTPUT_IMAGE"
printf 'Archive size: %s bytes\n' "$(wc -c < "$OUTPUT_IMAGE" | tr -d ' ')"
printf '%s\n' "Status: development initramfs only; device bootability is not verified."
