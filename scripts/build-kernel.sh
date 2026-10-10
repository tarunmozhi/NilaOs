#!/usr/bin/env bash
set -Eeuo pipefail

# Build from an already obtained, device-compatible Linux kernel tree.
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
KERNEL_SRC="${KERNEL_SRC:-}"
KERNEL_DEFCONFIG="${KERNEL_DEFCONFIG:-}"
OUT_DIR="${OUT_DIR:-$ROOT/out/kernel}"
JOBS="${JOBS:-$(getconf _NPROCESSORS_ONLN 2>/dev/null || printf '2')}"

usage() {
  cat <<'EOF'
Usage:
  KERNEL_SRC=/path/to/kernel KERNEL_DEFCONFIG=<device_defconfig> ./scripts/build-kernel.sh

Required:
  KERNEL_SRC          Linux kernel source tree matching the target device
  KERNEL_DEFCONFIG    Exact defconfig supplied by that kernel/device tree

Optional:
  OUT_DIR             Output directory (default: out/kernel)
  JOBS                Parallel build jobs (default: available CPU count)

A successful compile does not establish bootability: vendor modules, firmware,
device tree selection, boot image packaging, AVB and partition compatibility
must be handled and tested separately.
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then usage; exit 0; fi
if [[ -z "$KERNEL_SRC" || -z "$KERNEL_DEFCONFIG" ]]; then
  echo "ERROR: KERNEL_SRC and KERNEL_DEFCONFIG are required." >&2
  usage >&2
  exit 2
fi
if [[ ! -d "$KERNEL_SRC" || ! -f "$KERNEL_SRC/Makefile" ]]; then
  echo "ERROR: KERNEL_SRC must point to a Linux kernel source tree." >&2
  exit 2
fi
if [[ ! -f "$KERNEL_SRC/arch/arm64/configs/$KERNEL_DEFCONFIG" ]]; then
  echo "ERROR: defconfig not found: arch/arm64/configs/$KERNEL_DEFCONFIG" >&2
  exit 2
fi
if ! command -v make >/dev/null 2>&1; then
  echo "ERROR: make is required." >&2
  exit 2
fi
if ! command -v aarch64-linux-gnu-gcc >/dev/null 2>&1; then
  echo "ERROR: install an AArch64 GNU cross compiler (aarch64-linux-gnu-gcc)." >&2
  exit 2
fi

mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"
make -C "$KERNEL_SRC" O="$OUT_DIR" ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- "$KERNEL_DEFCONFIG"
make -C "$KERNEL_SRC" O="$OUT_DIR" ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- -j"$JOBS" Image.gz dtbs

if [[ ! -s "$OUT_DIR/arch/arm64/boot/Image.gz" ]]; then
  echo "ERROR: kernel build completed without Image.gz output." >&2
  exit 1
fi
printf 'Kernel build output: %s\n' "$OUT_DIR/arch/arm64/boot/Image.gz"
printf 'DTB output directory: %s\n' "$OUT_DIR/arch/arm64/boot/dts"
printf '%s\n' "Build completed. Device bootability is NOT verified."
