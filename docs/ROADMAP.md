# Nila OS Engineering Roadmap

## Target

Nila OS is a mobile-only deployment project, initially targeting vivo Y11 / vivo 1906 (PD1930F), Qualcomm Snapdragon 439 / SDM439, ARM64, 3 GB RAM, 32 GB eMMC, and a 720×1544 display. PC/Linux/VM environments are development and test hosts only.

## Milestone 0 — Safe engineering baseline

- [x] Rust workspace and GitHub Actions CI
- [x] Device profile schema validation and regression tests
- [x] Bounded-memory SHA-256 hashing
- [x] Ed25519 verification of manifest metadata against explicitly provisioned trusted public keys
- [x] Payload digest verification against the signed manifest
- [x] Fail-closed package authorization while application runtimes are absent
- [x] Guarded kernel build helper and build-input checks
- [x] Lockfile committed and CI enforces locked dependency resolution

## Milestone 1 — MHF package pipeline

- [ ] Define a versioned MHF container format, including canonical manifest encoding and payload layout
- [ ] Implement a streaming package reader with strict size limits and archive path traversal protection
- [ ] Add safe staging, fsync, atomic commit/rollback, and TOCTOU protection
- [ ] Add malformed-container fuzz tests and property tests
- [ ] Define release signing, key rotation, revocation, and recovery procedures

## Milestone 2 — Runtime and app compatibility

- [ ] Implement Maha Android Runtime (MAR) with Android API and ABI compatibility boundaries
- [ ] Implement an approved Linux runtime with strict sandboxing
- [ ] Keep EXE and APP recognition separate from translation or execution support
- [ ] Integrate per-UID isolation, SELinux domains, storage/network policies, and runtime resource limits
- [ ] Add runtime-specific conformance and security tests

## Milestone 3 — vivo 1906 device bring-up

- [ ] Obtain and verify the exact device-compatible SDM439 kernel source and PD1930F defconfig
- [ ] Identify matching device tree/DTBO, partition map, boot image layout, and vendor module requirements
- [ ] Integrate legally obtained Qualcomm vendor blobs, HALs, and firmware
- [ ] Build a recovery environment and verify a safe restore path
- [ ] Integrate Android framework/vendor configuration and hardware services
- [ ] Integrate AVB/verified boot and a complete enforcing SELinux policy
- [ ] Validate display, touch, modem, dual SIM, Wi-Fi, Bluetooth, GNSS, camera, audio, fingerprint, USB/OTG, and power management

## Milestone 4 — UI and installation

- [ ] Implement the Nila Skin mobile shell and SystemUI runtime
- [ ] Implement the graphical installer, compatibility gate, backup, verification, migration, and rollback
- [ ] Integrate Nila Security Center, Vault, and External Memory runtime
- [ ] Package a device-specific boot image only after kernel, recovery, partition, and integrity checks are verified

## Release gates

A passing Rust build is not a bootable operating system. Do not flash an image until all device-specific components, partition compatibility, verified boot, recovery, and rollback have been validated. A release requires a recorded boot test and hardware test on the actual supported device; no such phone test is currently claimed.
