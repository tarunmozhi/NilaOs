# Nila OS: path to a bootable vivo 1906 build

## Current truth

The current repository contains a Rust workspace and a reference-device profile. It
does **not** contain a Linux kernel source tree, a verified vivo 1906 kernel defconfig,
Qualcomm vendor blobs, Android HAL implementations, a recovery image, a boot image,
or a complete Android/AOSP product tree. It cannot currently produce a bootable phone
image on its own.

The target remains mobile-only: vivo Y11 / vivo 1906 / PD1930F, Qualcomm SDM439,
ARM64, 3 GB RAM. A PC or virtual machine is a development and testing host, not a
supported installation target.

## Required engineering stages

1. **Acquire a legally usable device baseline.** Identify the exact PD1930F firmware
   and bootloader state, partition map, kernel source and license, device tree,
   defconfig, vendor modules, firmware, and proprietary HALs. Verify source provenance
   and checksums. Do not substitute another SDM439 device's binaries without validation.
2. **Build the device kernel.** Use scripts/build-kernel.sh with a verified kernel
   source checkout and its exact device defconfig. Inspect the resulting kernel config,
   DTBs/DTBOs, modules, and required firmware. A compile is not a boot test.
3. **Integrate the Android hardware foundation.** Use a compatible AOSP/Android
   framework baseline, device/vendor configuration, init .rc files, fstab, VINTF
   manifests/matrices, SELinux policy, and HAL services. Reuse is possible only where
   licenses and ABI compatibility allow it.
4. **Build Nila userspace.** Integrate the Nila Rust services with Android init and
   SELinux domains; implement SystemUI/Settings and the runtime bridges. A Rust service
   registry is not an init system and does not itself enforce SELinux.
5. **Package and verify images.** Generate the correct boot/vendor_boot (where
   applicable), dtbo, vbmeta and system/vendor images for the device's actual partition
   scheme. Add recovery, signed update packages, hashes, and rollback/restore checks.
6. **Test safely in stages.** Run workspace and static checks first; then boot in a
   suitable emulator for supported components; then use a recoverable test device.
   Verify display/touch, storage, Wi-Fi, Bluetooth, audio, camera, sensors, telephony,
   GNSS, fingerprint, suspend/resume, charging and thermal behavior individually.
   Never flash an image just because compilation succeeded.

## Kernel build helper

On a Linux build host, install make, Python 3, Git, and an AArch64 GNU cross compiler.
Then use a device-compatible kernel source checkout:

    KERNEL_SRC=/absolute/path/to/kernel \
    KERNEL_DEFCONFIG=<verified_pd1930f_defconfig> \
    ./scripts/build-kernel.sh

Replace the placeholder only after confirming the exact defconfig exists in the
verified source tree. Do not guess the defconfig name.

## Checks available in this repository

    python3 scripts/validate-device-profile.py
    bash -n scripts/build-kernel.sh
    cargo fmt --all -- --check
    cargo test --workspace
    cargo clippy --workspace -- -D warnings

These validate metadata and existing Rust code; they do not create a bootable image.
The project must not be labelled bootable until a device-specific image has been
built and its boot and hardware behavior have been tested and documented.
