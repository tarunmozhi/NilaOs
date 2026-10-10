#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
script="$ROOT/scripts/inspect-kernel-source.sh"

bash -n "$script"
bash "$script" --help >/dev/null
if bash "$script" relative/path >/dev/null 2>&1; then
  echo "ERROR: relative source path was accepted" >&2; exit 1
fi
if bash "$script" "$tmp/missing" >/dev/null 2>&1; then
  echo "ERROR: missing source tree was accepted" >&2; exit 1
fi
mkdir -p "$tmp/fake/arch/arm64/configs" "$tmp/fake/arch/arm64/boot/dts" "$tmp/fake/drivers"
cat > "$tmp/fake/Makefile" <<'EOF'
VERSION = 4
PATCHLEVEL = 9
SUBLEVEL = 337
EXTRAVERSION =
EOF
: > "$tmp/fake/arch/arm64/configs/test_defconfig"
printf '/* compatible = "qcom,sdm439"; */\n' > "$tmp/fake/arch/arm64/boot/dts/test.dts"
output="$(bash "$script" "$tmp/fake")"
grep -q 'VERSION = 4' <<<"$output"
grep -q 'test_defconfig' <<<"$output"
grep -q 'test.dts' <<<"$output"
grep -q 'NOT verified' <<<"$output"
echo "Kernel source inspector guard tests passed."
