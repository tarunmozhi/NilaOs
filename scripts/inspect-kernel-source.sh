#!/usr/bin/env bash
set -Eeuo pipefail

usage() {
  cat <<'EOF'
Usage:
  bash scripts/inspect-kernel-source.sh /absolute/path/to/linux-source

Read-only inspection of a candidate Linux kernel tree for Nila's vivo 1906
(SDM439, ARM64) target. This script does not download, modify, configure, or
build kernel source. A match is only a lead for manual verification, not proof
that a tree can boot the device.
EOF
}
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then usage; exit 0; fi
if [[ "$#" -ne 1 ]]; then usage >&2; exit 2; fi
src="$1"
if [[ "$src" != /* ]]; then
  echo "ERROR: source path must be absolute." >&2
  exit 2
fi
if [[ ! -d "$src" || ! -f "$src/Makefile" || ! -d "$src/arch/arm64" ]]; then
  echo "ERROR: expected a Linux source tree with Makefile and arch/arm64/." >&2
  exit 2
fi
src="$(cd -- "$src" && pwd -P)"
echo "Kernel source: $src"
if grep -Eq '^[[:space:]]*VERSION[[:space:]]*=[[:space:]]*[0-9]+' "$src/Makefile"; then
  grep -E '^[[:space:]]*(VERSION|PATCHLEVEL|SUBLEVEL|EXTRAVERSION)[[:space:]]*=' "$src/Makefile" | head -n 4
else
  echo "WARNING: could not read a conventional kernel version from Makefile."
fi
printf '\nARM64 defconfigs: '
if [[ -d "$src/arch/arm64/configs" ]]; then
  find "$src/arch/arm64/configs" -maxdepth 1 -type f -name '*defconfig' -printf '%f ' | sort | head -c 1000
  printf '\n'
else
  echo "MISSING"
fi
printf '\nDevice/SoC source references (bounded sample):\n'
if command -v grep >/dev/null 2>&1; then
  grep -RIlE 'sdm439|PD1930F|vivo.?1906|1906' \
    "$src/arch/arm64/boot/dts" "$src/arch/arm64/configs" "$src/drivers" \
    2>/dev/null | head -n 30 || true
fi
printf '\nNext checks before building:\n'
printf '%s\n' '  1. Confirm upstream/provenance and exact device support.' \
  '  2. Identify the exact defconfig, DTS/DTBO, required vendor modules and firmware.' \
  '  3. Verify boot image format, partition layout, AVB/signing and recovery path.' \
  '  4. Build only after those inputs are confirmed; do not flash an unverified image.'
echo "Inspection complete; device compatibility is NOT verified."
