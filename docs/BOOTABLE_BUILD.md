# Nila OS: path to a bootable Linux build for vivo 1906

## Current truth

Nila OS targets a Linux kernel and native Linux userspace. Android/Funtouch values identify the phone's factory software; they do not make Nila an Android or AOSP system.

The GitHub repository is an engineering baseline, not a bootable phone image. It does not contain the device kernel source, a native Linux root filesystem, early userspace, an init/service manager, a recovery image, or phone-image packaging.

## Candidate kernel source inspected

A user-provided local checkout outside this repository was inspected read-only. It reports Linux 4.9.227, and its `.git` metadata identifies `vivo-source-mirror/Y11` at commit `9c04dc87d779a852c62cbd2605a725fc0295b99f`. The remote's name and the matching files make it a promising candidate, but do not independently prove Vivo provenance.

The candidate contains `msm8937_defconfig`, which enables `CONFIG_ARCH_SDM439=y`, and device-tree overlays named `sdm439-mtp-overlay-PD1930F_EX` and `sdm439-mtp-overlay-PD1930BF_EX`. The reported stock build is `PD1930CF_EX_A_6.71.15`; no overlay is explicitly named `PD1930CF`. Do not assume the CF build ID maps to either overlay until the exact device/board ID is confirmed. The [official Y11 update page](https://www.vivo.com/in/support/upgradePackageData?id=82) lists a PD1930F package, which is useful evidence but does not establish the mapping for the reported CF build.

The candidate checkout's build note expects an `aarch64-linux-android-4.9` toolchain and names `Image.gz-dtb` as its kernel output. The Nila kernel helper now accepts an explicit `CROSS_COMPILE` prefix and builds that output as a **kernel candidate**. The candidate source is not checked into Nila's repository, and no kernel compile or phone boot has been performed here.

## Required engineering stages

1. **Confirm the exact device identity.** Record the phone's model/product identifiers, full stock build string, board ID, and bootloader state from the device and its stock firmware. Match those values against the source tree and selected overlay. Keep `PD1930F`, `PD1930BF`, and reported `PD1930CF` identifiers distinct until verified.
2. **Verify source and configuration provenance.** Preserve the kernel source revision and license, inspect the exact defconfig and selected DTS/DTBO, and compare them with the stock device's kernel/DT metadata. The current source checkout has a generic SDM439 defconfig and candidate overlays; this is not yet proof of a CF-compatible configuration.
3. **Build a kernel candidate.** On a Linux build host with the required cross compiler, build out of tree. Inspect the resulting configuration, `Image.gz-dtb`, DTBs/DTBOs, modules, and firmware. A successful compile is not a boot test.
4. **Create native Linux userspace.** Select a Linux userspace composition and init/service manager. Build the root filesystem, early-boot services, mount and storage configuration, networking, logging, and Nila services. Android framework and AOSP services are not part of the Nila base.
5. **Bring up hardware.** Integrate and test Linux drivers and firmware for display, touch, storage, Wi-Fi, Bluetooth, audio, camera, sensors, telephony, GNSS, fingerprint, USB/OTG, suspend/resume, charging, and thermal behavior. Verify provenance, license, and interfaces for any proprietary firmware.
6. **Implement the Nila graphical session.** Run Nila Skin as a native Linux shell using the selected Linux display/input stack and Nila service interfaces. The browser prototype is not a native system UI.
7. **Package and verify images.** Match the exact bootloader and partition layout. Add recovery, integrity checks, signed updates, and a tested rollback/restore path. A boot-image container required by the device boot chain does not make the userspace Android.
8. **Test safely in stages.** Run workspace and static checks, then boot supported components in an appropriate emulator, then test on a recoverable device. Do not flash an image solely because compilation succeeded.

## Candidate kernel command

The local source's build note uses `msm8937_defconfig`. After confirming it is the right configuration for the exact phone variant and obtaining the matching Android AArch64 toolchain, the helper can be called like this from a Nila OS checkout:

```bash
KERNEL_SRC=/path/to/vivo-y11-kernel \
KERNEL_DEFCONFIG=msm8937_defconfig \
CROSS_COMPILE=/path/to/aarch64-linux-android-4.9/bin/aarch64-linux-android- \
OUT_DIR=/path/to/separate/kernel-output \
./scripts/build-kernel.sh
```

The helper writes `Image.gz-dtb` to the output directory. This is not a Nila OS image and must not be treated as flash-ready.

## Checks available in the Nila repository

```bash
python3 scripts/validate-device-profile.py
python3 -m unittest discover -s tests -p 'test_*.py'
bash -n scripts/build-kernel.sh tests/test-build-kernel-helper.sh
bash tests/test-build-kernel-helper.sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

These checks validate metadata, the Rust baseline, and the helper's input/output behavior with mocked tools. They do not build the external kernel source or create a bootable Linux image. Nila must not be described as bootable until a device-specific Linux image has been built and its boot and hardware behavior tested and documented.
