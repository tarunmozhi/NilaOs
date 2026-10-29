#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT

bash -n "$ROOT/scripts/package-native-rootfs.sh"
bash "$ROOT/scripts/package-native-rootfs.sh" --help >/dev/null

MOCK_BIN="$TEMP_ROOT/mock-bin"
ROOTFS="$TEMP_ROOT/rootfs"
OUTPUT="$TEMP_ROOT/nila-initramfs.cpio.gz"
mkdir -p "$MOCK_BIN" "$ROOTFS/bin"
cat > "$MOCK_BIN/file" <<'EOF'
#!/usr/bin/env bash
echo "ELF 64-bit LSB executable, ARM aarch64, statically linked"
EOF
chmod +x "$MOCK_BIN/file"
printf '#!/bin/sh\n# mock BusyBox; not executed\n' > "$ROOTFS/bin/busybox"
printf '#!/bin/sh\nexec /bin/sh\n' > "$ROOTFS/init"
chmod +x "$ROOTFS/bin/busybox" "$ROOTFS/init"
touch "$ROOTFS/.nila-rootfs-staging"

PATH="$MOCK_BIN:$PATH" ROOTFS_DIR="$ROOTFS" OUTPUT_IMAGE="$OUTPUT" \
  bash "$ROOT/scripts/package-native-rootfs.sh" >/dev/null
gzip -t "$OUTPUT"
gzip -dc "$OUTPUT" | cpio -it 2>/dev/null > "$TEMP_ROOT/archive.list"
grep -Fx 'init' "$TEMP_ROOT/archive.list" >/dev/null
grep -Fx 'bin/busybox' "$TEMP_ROOT/archive.list" >/dev/null
grep -Fx '.nila-rootfs-staging' "$TEMP_ROOT/archive.list" >/dev/null
[[ -s "$OUTPUT" ]]

mkdir -p "$TEMP_ROOT/unmarked"
printf 'keep-me\n' > "$TEMP_ROOT/unmarked/user-data.txt"
if PATH="$MOCK_BIN:$PATH" ROOTFS_DIR="$TEMP_ROOT/unmarked" OUTPUT_IMAGE="$OUTPUT" \
   bash "$ROOT/scripts/package-native-rootfs.sh" >/dev/null 2>&1; then
  echo "ERROR: unmarked directory was packaged" >&2
  exit 1
fi
grep -Fx 'keep-me' "$TEMP_ROOT/unmarked/user-data.txt" >/dev/null

echo "Native rootfs packaging tests passed with a mock ARM64 binary; no device boot was tested."
