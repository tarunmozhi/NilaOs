# Native Linux userspace bootstrap (experimental)

The repository now has an opt-in helper that can create a **generic Debian Bookworm ARM64 development root filesystem** on a Linux/WSL host. It is a foundation experiment only, not a Nila OS release artifact.

## Run on Ubuntu/WSL

Install the host tools first:

```bash
sudo apt-get update
sudo apt-get install debootstrap qemu-user-static
```

Then from the repository root:

```bash
sudo env ROOTFS_DIR="$PWD/out/rootfs-arm64" bash scripts/build-rootfs.sh
```

The script refuses to overwrite an existing destination and requires an HTTPS mirror. To check host prerequisites without building:

```bash
sudo bash scripts/build-rootfs.sh --check
```

## Explicit limitations

This prototype does not select or build a vivo 1906 kernel, add device drivers/firmware, configure the target partition map, implement Nila's init/service manager, or create a boot image. It does not modify bootloader state or flash any device. The rootfs is not ready to boot on the phone.

Before this can become a device image, the project must verify the exact PD1930CF/PD1930F board mapping, kernel source provenance and defconfig, boot chain and DTB/DTBO, required firmware, recovery and rollback, security policy, and device-specific service startup. A generic ARM64 rootfs is not evidence of hardware compatibility.
