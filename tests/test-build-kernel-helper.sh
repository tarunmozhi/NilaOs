#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
TEMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TEMP_ROOT"' EXIT
KERNEL_SRC="$TEMP_ROOT/kernel"
TOOL_BIN="$TEMP_ROOT/toolchain/bin"
OUT_DIR="$TEMP_ROOT/output"
CROSS_COMPILE="$TOOL_BIN/aarch64-linux-android-"
MAKE_LOG="$TEMP_ROOT/make.log"

mkdir -p "$KERNEL_SRC/arch/arm64/configs" "$TOOL_BIN"
touch "$KERNEL_SRC/Makefile" "$KERNEL_SRC/arch/arm64/configs/test_defconfig"

cat > "$TOOL_BIN/aarch64-linux-android-gcc" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
cat > "$TOOL_BIN/make" <<'EOF'
#!/usr/bin/env bash
set -Eeuo pipefail
printf '%s\n' "$*" >> "$MAKE_LOG"
output=""
for arg in "$@"; do
  case "$arg" in
    O=*) output="${arg#O=}" ;;
  esac
done
case " $* " in
  *" test_defconfig "*) exit 0 ;;
  *" Image.gz-dtb "*)
    [[ -n "$output" ]]
    mkdir -p "$output/arch/arm64/boot"
    printf 'mock-kernel\n' > "$output/arch/arm64/boot/Image.gz-dtb"
    exit 0
    ;;
  *)
    echo "unexpected make target: $*" >&2
    exit 1
    ;;
esac
EOF
chmod +x "$TOOL_BIN/aarch64-linux-android-gcc" "$TOOL_BIN/make"

PATH="$TOOL_BIN:$PATH" \
MAKE_LOG="$MAKE_LOG" \
KERNEL_SRC="$KERNEL_SRC" \
KERNEL_DEFCONFIG="test_defconfig" \
CROSS_COMPILE="$CROSS_COMPILE" \
OUT_DIR="$OUT_DIR" \
JOBS=2 \
bash "$ROOT/scripts/build-kernel.sh"

[[ -s "$OUT_DIR/arch/arm64/boot/Image.gz-dtb" ]]
grep -F "CROSS_COMPILE=$CROSS_COMPILE" "$MAKE_LOG" >/dev/null
grep -F "Image.gz-dtb" "$MAKE_LOG" >/dev/null

if PATH="$TOOL_BIN:$PATH" \
   KERNEL_SRC="$KERNEL_SRC" \
   KERNEL_DEFCONFIG="test_defconfig" \
   CROSS_COMPILE='invalid;command-' \
   OUT_DIR="$TEMP_ROOT/invalid-output" \
   bash "$ROOT/scripts/build-kernel.sh" >/dev/null 2>&1; then
  echo "ERROR: unsafe CROSS_COMPILE value was accepted" >&2
  exit 1
fi

echo "Kernel build helper mock passed; no kernel source was compiled."
