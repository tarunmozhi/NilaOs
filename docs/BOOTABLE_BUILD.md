# Nila OS: path to a bootable Linux build for vivo 1906

## Current truth

Nila OS targets a Linux kernel and native Linux userspace. Android/Funtouch details in the device profile identify the phone's factory firmware; they do not make Nila an Android or AOSP system.

The current repository contains a Rust engineering baseline and a reference-device profile. It does **not** contain a verified vivo 1906 Linux kernel source tree and configuration, a native Linux root filesystem, early userspace, an init/service manager, a recovery image, or a phone image. It cannot currently produce a bootable Nila image on its own.

The initial device target is vivo Y11 / vivo 1906 / PD1930F, Qualcomm SDM439, ARM64, 3 GB RAM. A PC or virtual machine is a development and testing host, not a supported installation target.

## Required engineering stages

1. **Verify the device baseline.** Record the exact PD1930F firmware and bootloader state, partition map, kernel source and license, device tree, configuration, kernel modules, and firmware. Verify source provenance and checksums. Do not substitute another SDM439 device's binaries without validation.
2. **Build the Linux kernel.** Use `scripts/build-kernel.sh` with a verified kernel source checkout and its exact device configuration. Inspect the resulting kernel config, DTBs/DTBOs, modules, and required firmware. A compile is not a boot test.
3. **Create native Linux userspace.** Select a Linux userspace composition and init/service manager. Build the root filesystem, early-boot services, mount and storage configuration, networking, logging, and the Nila system services. Android framework and AOSP services are not required for this Linux base.
4. **Bring up hardware.** Integrate and test Linux drivers and firmware for display, touch, storage, Wi-Fi, Bluetooth, audio, camera, sensors, telephony, GNSS, fingerprint, USB/OTG, suspend/resume, charging, and thermal behavior. Reuse proprietary firmware or vendor components only when their provenance, license, and interfaces are verified.
5. **Implement the Nila graphical session.** Run the Nila mobile shell and settings against native Linux display/input and Nila service interfaces. The browser Skin prototype is not a native system UI.
6. **Package and verify images.** Match the actual bootloader and partition layout. A boot image container may be required by the phone's existing boot chain; using that container format does not make Nila an Android userspace. Add recovery, signed updates, integrity checks, and rollback/restore behavior.
7. **Test safely in stages.** Run workspace and static checks first, then boot supported components in a suitable emulator, then test on a recoverable device. Never flash an image merely because compilation succeeded.

## Kernel build helper

On a Linux build host, install make, Git, and an AArch64 GNU cross compiler. Then use a verified device-compatible Linux kernel source checkout:

    KERNEL_SRC=/absolute/path/to/kernel \
    KERNEL_DEFCONFIG=<verified_pd1930f_defconfig> \
    ./scripts/build-kernel.sh

Replace the placeholder only after confirming the exact configuration exists in the verified source tree. Do not guess its name.

## Checks available in this repository

    python3 scripts/validate-device-profile.py
    bash -n scripts/build-kernel.sh
    cargo fmt --all -- --check
    cargo test --workspace
    cargo clippy --workspace -- -D warnings

These validate metadata and existing Rust code; they do not create a bootable Linux image. Do not label Nila bootable until a device-specific image has been built and its boot and hardware behavior tested and documented.
