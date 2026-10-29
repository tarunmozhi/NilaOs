#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT

bash -n "$ROOT/scripts/setup-native-rootfs.sh"
bash "$ROOT/scripts/setup-native-rootfs.sh" --help >/dev/null

if BUSYBOX_STATIC="$TEMP_ROOT/missing-busybox" \
   ROOTFS_DIR="$TEMP_ROOT/missing-rootfs" \
   bash "$ROOT/scripts/setup-native-rootfs.sh" >/dev/null 2>&1; then
  echo "ERROR: missing BusyBox was accepted" >&2
  exit 1
fi

# Failed validation must not remove or overwrite an existing user directory.
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

# Mock file(1) output to exercise staging layout without pretending the mock is
# a real ARM64 binary or attempting to execute it.
MOCK_BIN="$TEMP_ROOT/mock-bin"
mkdir -p "$MOCK_BIN"
cat > "$MOCK_BIN/file" <<'EOF'
#!/usr/bin/env bash
echo "ELF 64-bit LSB executable, ARM aarch64, statically linked"
EOF
chmod +x "$MOCK_BIN/file"
printf '#!/bin/sh\n# mock only; never executed\n' > "$TEMP_ROOT/mock-busybox"
chmod +x "$TEMP_ROOT/mock-busybox"

PATH="$MOCK_BIN:$PATH" \
BUSYBOX_STATIC="$TEMP_ROOT/mock-busybox" \
ROOTFS_DIR="$TEMP_ROOT/rootfs" \
bash "$ROOT/scripts/setup-native-rootfs.sh" >/dev/null

[[ -x "$TEMP_ROOT/rootfs/init" ]]
[[ -x "$TEMP_ROOT/rootfs/bin/busybox" ]]
[[ -L "$TEMP_ROOT/rootfs/bin/sh" ]]
[[ -L "$TEMP_ROOT/rootfs/sbin/init" ]]
grep -F 'Development rescue shell only' "$TEMP_ROOT/rootfs/init" >/dev/null
grep -F 'STATUS="development-scaffold-not-a-phone-image"' "$TEMP_ROOT/rootfs/etc/nila-release" >/dev/null

# The overwrite guard may replace only a directory created by this helper.
printf 'test marker\n' > "$TEMP_ROOT/rootfs/user-test-file"
if PATH="$MOCK_BIN:$PATH" \
   BUSYBOX_STATIC="$TEMP_ROOT/mock-busybox" \
   ROOTFS_DIR="$TEMP_ROOT/rootfs" \
   bash "$ROOT/scripts/setup-native-rootfs.sh" >/dev/null 2>&1; then
  echo "ERROR: existing staging rootfs was overwritten without opt-in" >&2
  exit 1
fi
[[ -f "$TEMP_ROOT/rootfs/user-test-file" ]]
PATH="$MOCK_BIN:$PATH" \
BUSYBOX_STATIC="$TEMP_ROOT/mock-busybox" \
ROOTFS_DIR="$TEMP_ROOT/rootfs" \
NILA_ROOTFS_OVERWRITE=1 \
bash "$ROOT/scripts/setup-native-rootfs.sh" >/dev/null
[[ -f "$TEMP_ROOT/rootfs/.nila-rootfs-staging" ]]
[[ ! -f "$TEMP_ROOT/rootfs/user-test-file" ]]

echo "Native rootfs setup tests passed using a mock binary; no ARM64 rootfs was built and no device was modified."
