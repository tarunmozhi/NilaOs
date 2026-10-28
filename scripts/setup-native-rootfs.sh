#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
BUSYBOX_STATIC="${BUSYBOX_STATIC:-}"
ROOTFS_DIR="${ROOTFS_DIR:-$ROOT/out/native-rootfs}"
NILA_ROOTFS_OVERWRITE="${NILA_ROOTFS_OVERWRITE:-0}"

usage() {
  cat <<'EOF'
Usage:
  BUSYBOX_STATIC=/path/to/aarch64-static-busybox ./scripts/setup-native-rootfs.sh

Required:
  BUSYBOX_STATIC   Executable, statically linked ARM64 BusyBox binary built for the target

Optional:
  ROOTFS_DIR       Output staging directory (default: out/native-rootfs)
  NILA_ROOTFS_OVERWRITE=1
                   Allow replacing an existing directory marked .nila-rootfs-staging

This creates a minimal native-Linux initramfs/rootfs staging tree for early bring-up.
It is NOT a complete phone root filesystem, boot image, or flashable image. It does
not mount Android partitions, select a device tree, load vendor firmware, or start
unimplemented Nila services. The init script mounts basic virtual filesystems and
opens a rescue shell on the console.
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then usage; exit 0; fi
if [[ -z "$BUSYBOX_STATIC" || ! -f "$BUSYBOX_STATIC" || ! -x "$BUSYBOX_STATIC" ]]; then
  echo "ERROR: BUSYBOX_STATIC must point to an executable static BusyBox binary." >&2
  usage >&2
  exit 2
fi
if ! command -v file >/dev/null 2>&1; then
  echo "ERROR: 'file' is required to validate the BusyBox architecture." >&2
  exit 2
fi
description="$(file -Lb "$BUSYBOX_STATIC")"
case "$description" in
  *ARM*aarch64*|*ARM64*|*aarch64*)
    ;;
  *)
    echo "ERROR: BusyBox does not appear to be an ARM64 binary: $description" >&2
    exit 2
    ;;
esac
case "$description" in
  *statically linked*) ;;
  *)
    echo "ERROR: BusyBox must be statically linked for early userspace: $description" >&2
    exit 2
    ;;
esac

mkdir -p "$(dirname -- "$ROOTFS_DIR")"
ROOTFS_DIR="$(cd -- "$(dirname -- "$ROOTFS_DIR")" && pwd -P)/$(basename -- "$ROOTFS_DIR")"
if [[ -e "$ROOTFS_DIR" ]]; then
  if [[ "$NILA_ROOTFS_OVERWRITE" != "1" || ! -f "$ROOTFS_DIR/.nila-rootfs-staging" ]]; then
    echo "ERROR: output exists and is not an explicitly replaceable Nila staging tree: $ROOTFS_DIR" >&2
    exit 2
  fi
  rm -rf -- "$ROOTFS_DIR"
fi

mkdir -p "$ROOTFS_DIR"/{bin,sbin,etc/init.d,dev,proc,sys,run,tmp,var/log,var/tmp,root,home,nila}
chmod 0755 "$ROOTFS_DIR" "$ROOTFS_DIR/bin" "$ROOTFS_DIR/sbin"
install -m 0755 "$BUSYBOX_STATIC" "$ROOTFS_DIR/bin/busybox"

for applet in sh mount umount mkdir mdev dmesg cat echo sleep; do
  ln -sfn busybox "$ROOTFS_DIR/bin/$applet"
done
ln -sfn ../bin/busybox "$ROOTFS_DIR/sbin/init"

cat > "$ROOTFS_DIR/init" <<'EOF'
#!/bin/sh
# Minimal native Linux early userspace for development bring-up only.
export PATH=/bin:/sbin
export HOME=/root
export TERM=linux

mount -t proc proc /proc 2>/dev/null || :
mount -t sysfs sysfs /sys 2>/dev/null || :
mount -t devtmpfs devtmpfs /dev 2>/dev/null || :
mount -t devpts devpts /dev/pts 2>/dev/null || :
mount -t tmpfs -o mode=0755,nosuid,nodev tmpfs /run 2>/dev/null || :
mkdir -p /dev/pts /dev/shm /run /tmp /var/log
chmod 1777 /tmp
if [ -x /bin/mdev ]; then
  echo /sbin/mdev > /proc/sys/kernel/hotplug 2>/dev/null || :
  mdev -s 2>/dev/null || :
fi

echo
echo "Nila OS native Linux early userspace"
echo "Development rescue shell only; full rootfs and hardware support are not installed."
echo "No Android partitions are mounted or modified."
echo
exec /bin/sh </dev/console >/dev/console 2>&1
EOF
chmod 0755 "$ROOTFS_DIR/init"

cat > "$ROOTFS_DIR/etc/nila-release" <<'EOF'
NAME="Nila OS"
ID=nila
PRETTY_NAME="Nila OS Native Linux Bring-up RootFS"
VERSION_ID="0.0.1-dev"
STATUS="development-scaffold-not-a-phone-image"
EOF

cat > "$ROOTFS_DIR/etc/inittab" <<'EOF'
# PID 1 is /init for this early bring-up scaffold.
# No getty, networking, or production service manager is configured.
EOF

touch "$ROOTFS_DIR/.nila-rootfs-staging"
printf 'Native Linux rootfs staging tree created: %s\n' "$ROOTFS_DIR"
printf 'BusyBox: %s\n' "$description"
printf '%s\n' "Validation: staging files only; this is not a complete or flashable OS image."
