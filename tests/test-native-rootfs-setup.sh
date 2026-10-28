#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT

bash -n "$ROOT/scripts/setup-native-rootfs.sh"
bash "$ROOT/scripts/setup-native-rootfs.sh" --help >/dev/null

if BUSYBOX_STATIC="$TEMP_ROOT/missing-busybox" \
   ROOTFS_DIR="$TEMP_ROOT/rootfs" \
   bash "$ROOT/scripts/setup-native-rootfs.sh" >/dev/null 2>&1; then
  echo "ERROR: missing BusyBox was accepted" >&2
  exit 1
fi

mkdir -p "$TEMP_ROOT/existing"
printf 'preserve-me\n' > "$TEMP_ROOT/existing/user-data.txt"
if BUSYBOX_STATIC="$TEMP_ROOT/missing-busybox" \
   ROOTFS_DIR="$TEMP_ROOT/existing" \
   NILA_ROOTFS_OVERWRITE=1 \
   bash "$ROOT/scripts/setup-native-rootfs.sh" >/dev/null 2>&1; then
  echo "ERROR: missing BusyBox unexpectedly passed validation" >&2
  exit 1
fi
[[ -f "$TEMP_ROOT/existing/user-data.txt" ]]
grep -Fx 'preserve-me' "$TEMP_ROOT/existing/user-data.txt" >/dev/null

echo "Native rootfs setup guard tests passed; no rootfs or device was modified."
