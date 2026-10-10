#!/usr/bin/env bash
set -Eeuo pipefail

# Experimental Debian ARM64 userspace bootstrap for development only.
# This does NOT build a kernel, device tree, boot image, or flash a device.
ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
SUITE="${SUITE:-bookworm}"
MIRROR="${MIRROR:-https://deb.debian.org/debian}"
ROOTFS_DIR="${ROOTFS_DIR:-${HOME:-/tmp}/nila-rootfs-arm64}"
ARCH=arm64

usage() {
  cat <<'EOF'
Usage:
  sudo env ROOTFS_DIR=/absolute/output/path bash scripts/build-rootfs.sh

Optional environment:
  SUITE       Debian suite (default: bookworm)
  MIRROR      Debian mirror (default: https://deb.debian.org/debian)
  ROOTFS_DIR  New output directory (default: <repo>/out/rootfs-arm64)

Requirements:
  debootstrap, qemu-aarch64-static, chroot, and root privileges.

Safety:
  - refuses to overwrite an existing output directory
  - writes only to ROOTFS_DIR
  - does not mount partitions, modify bootloader state, or flash a device
  - produces a generic ARM64 development userspace, NOT a device-ready rootfs
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then usage; exit 0; fi
if [[ "${1:-}" == "--check" ]]; then
  command -v debootstrap >/dev/null 2>&1 || { echo "ERROR: debootstrap is required." >&2; exit 2; }
  command -v chroot >/dev/null 2>&1 || { echo "ERROR: chroot is required." >&2; exit 2; }
  command -v qemu-aarch64-static >/dev/null 2>&1 || { echo "ERROR: qemu-aarch64-static is required." >&2; exit 2; }
  if [[ "$(id -u)" -ne 0 ]]; then echo "ERROR: run with root privileges (for example, sudo)." >&2; exit 2; fi
  printf 'Prerequisites present. This check does not build or test a device image.\n'
  exit 0
fi

if [[ "$SUITE" != "bookworm" ]]; then
  echo "ERROR: only the reviewed default suite 'bookworm' is currently supported." >&2
  exit 2
fi
if [[ "$MIRROR" != https://* ]]; then
  echo "ERROR: MIRROR must use HTTPS." >&2
  exit 2
fi
if [[ "$ROOTFS_DIR" != /* ]]; then
  echo "ERROR: ROOTFS_DIR must be an absolute path." >&2
  exit 2
fi
case "$ROOTFS_DIR" in
  /|/bin|/boot|/dev|/etc|/home|/lib|/lib64|/proc|/root|/run|/sbin|/sys|/tmp|/usr|/var|"$ROOT"|"$ROOT/"*)
    echo "ERROR: refusing unsafe ROOTFS_DIR: $ROOTFS_DIR" >&2
    exit 2
    ;;
esac
if [[ -e "$ROOTFS_DIR" ]]; then
  echo "ERROR: ROOTFS_DIR already exists; refusing to overwrite it: $ROOTFS_DIR" >&2
  exit 2
fi
if [[ "$(id -u)" -ne 0 ]]; then
  echo "ERROR: run with root privileges (for example, sudo)." >&2
  exit 2
fi
for tool in debootstrap chroot qemu-aarch64-static; do
  command -v "$tool" >/dev/null 2>&1 || { echo "ERROR: required tool not found: $tool" >&2; exit 2; }
done

mkdir -p "$(dirname -- "$ROOTFS_DIR")"
debootstrap --arch="$ARCH" --foreign --variant=minbase "$SUITE" "$ROOTFS_DIR" "$MIRROR"
install -m 0755 "$(command -v qemu-aarch64-static)" "$ROOTFS_DIR/usr/bin/qemu-aarch64-static"
chroot "$ROOTFS_DIR" /debootstrap/debootstrap --second-stage
rm -f "$ROOTFS_DIR/usr/bin/qemu-aarch64-static"

cat > "$ROOTFS_DIR/etc/hostname" <<'EOF'
nila
EOF
cat > "$ROOTFS_DIR/etc/hosts" <<'EOF'
127.0.0.1 localhost
127.0.1.1 nila
::1 localhost ip6-localhost ip6-loopback
EOF
cat > "$ROOTFS_DIR/etc/apt/sources.list" <<EOF
deb $MIRROR $SUITE main
deb $MIRROR $SUITE-updates main
deb https://security.debian.org/debian-security $SUITE-security main
EOF

printf '\nGeneric ARM64 userspace created at: %s\n' "$ROOTFS_DIR"
printf '%s\n' 'NOT device-ready: no Nila init/services, device drivers, firmware, partition layout, recovery, verified boot, or kernel are included.'
printf '%s\n' 'Do not flash this directory. Device-specific integration and a recovery-tested image build are still required.'
