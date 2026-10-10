# Nila OS Engineering Roadmap

## Target architecture

Nila OS is a mobile-only, Linux-based operating-system project. The intended system is a Linux kernel with a native Linux userspace and Nila mobile shell; it is not Android or an AOSP distribution. The vivo Y11 / vivo 1906 (PD1930F), Qualcomm Snapdragon 439 / SDM439, ARM64, 3 GB RAM, 32 GB eMMC, and 720×1544 display are the initial hardware target. Its installed Android/Funtouch firmware is a device reference only. PC/Linux/VM environments are development and test hosts.

See [Target Architecture](ARCHITECTURE.md) for the system boundary and current implementation limits.

## Milestone 0 — Safe engineering baseline

- [x] Rust workspace and GitHub Actions CI
- [x] Device profile schema validation and regression tests
- [x] Bounded-memory SHA-256 hashing
- [x] Ed25519 verification of manifest metadata against explicitly provisioned trusted public keys
- [x] Payload digest verification against the signed manifest
- [x] Fail-closed package authorization while application runtimes are absent
- [x] Guarded kernel build helper and build-input checks
- [x] Lockfile committed and CI enforces locked dependency resolution

## Milestone 1 — Linux system foundation

- [ ] Select and document the Linux userspace, init/service manager, and base system composition
- [ ] Define the native root filesystem layout, boot-time service startup, logs, storage, and update model
- [ ] Add a reproducible build that combines the verified device kernel with the native Linux root filesystem
- [ ] Establish a recovery and rollback design before writing device partitions

## Milestone 2 — MHF package pipeline and native applications

- [ ] Define a versioned MHF container format, including canonical manifest encoding and payload layout
- [ ] Implement a streaming package reader with strict size limits and archive path traversal protection
- [ ] Add safe staging, fsync, atomic commit/rollback, and TOCTOU protection
- [ ] Add malformed-container fuzz tests and property tests
- [ ] Define release signing, key rotation, revocation, and recovery procedures
- [ ] Implement an approved native Linux application runtime with strict sandboxing
- [ ] Keep Android APK compatibility as a separate optional runtime; it is not part of the Linux boot or base userland
- [ ] Keep EXE and APP recognition separate from translation or execution support
- [ ] Integrate per-UID isolation, Linux security policy, storage/network policies, and runtime resource limits

## Milestone 3 — vivo 1906 Linux bring-up

- [ ] Obtain and verify the exact device-compatible Linux kernel source and PD1930F configuration
- [ ] Identify the bootloader path, device tree/DTBO, partition map, boot image layout, and required kernel modules
- [ ] Build early userspace and a native Linux root filesystem with device-specific mounts and service startup
- [ ] Integrate kernel drivers and legally obtained firmware needed for the target hardware; use closed vendor components only where required and validated
- [ ] Implement recovery and prove a safe restore path
- [ ] Integrate verified boot where the device boot chain supports a verifiable Nila image
- [ ] Implement and enforce the selected Linux security policy
- [ ] Validate display, touch, modem, dual SIM, Wi-Fi, Bluetooth, GNSS, camera, audio, fingerprint, USB/OTG, and power management

An Android framework or AOSP product tree is not a prerequisite for the Nila Linux base. Stock firmware, Android-derived firmware, or vendor components may be examined as hardware references or used as separately validated components when licensing and compatibility allow.

## Milestone 4 — Linux mobile UI and installation

- [ ] Implement Nila Skin as the shell and system UI for Nila's Linux graphical session
- [ ] Connect UI actions to permission-checked native Nila services
- [ ] Implement the installer, compatibility gate, backup, verification, migration, and rollback
- [ ] Integrate Nila Security Center, Vault, and External Memory with native services
- [ ] Package a device-specific Linux boot and root filesystem image only after kernel, recovery, partition, and integrity checks are verified

## Release gates

A passing Rust build is not a bootable operating system. Do not flash an image until all device-specific components, partition compatibility, verified boot, recovery, and rollback have been validated. A release requires a recorded boot test and hardware test on the actual supported device; no such phone test is currently claimed.
