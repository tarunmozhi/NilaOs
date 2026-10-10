# Nila OS Engineering Roadmap

## Project target
Mobile-first OS for vivo Y11 / vivo 1906 (PD1930F), Qualcomm Snapdragon 439 / SDM439, ARM64-capable Cortex-A53 platform, 3 GB RAM, 32 GB eMMC, 720x1544 display. PC/VM is a development and test environment, not a deployment target.

## Milestone 0 — Rust foundation
- [x] Rust workspace, formatting/build/test/lint CI
- [x] Streaming SHA-256 for large payloads
- [x] Explicitly report recognized-but-unimplemented formats as unsupported
- [x] Ed25519 signature verification against explicitly trusted public keys
- [x] Combined CLI check for signed manifest metadata and payload digest
- [x] Keep authorization fail-closed while runtimes are absent
- [ ] Production key lifecycle, release signing, key rotation and revocation
- [ ] Formal threat model and security review

## Milestone 1 — Package pipeline
- [ ] Define a versioned MHF container specification (canonical metadata encoding, payload layout, size limits, compression policy)
- [ ] Implement a streaming package reader with strict bounds and archive path traversal protection
- [ ] Bind manifest, exact payload bytes, package identity, version and runtime requirements into the signature
- [ ] Implement safe install staging, fsync, atomic commit/rollback and TOCTOU protection
- [ ] Add malformed-container fuzz tests and property tests
- [ ] Add a secure signing tool that reads keys from an OS key store or offline hardware-backed process, never from source control

## Milestone 2 — Runtime and app model
- [ ] Define MFR runtime adapter interface and capability declarations
- [ ] Implement MAR only after Android framework compatibility decisions and source integration
- [ ] Implement Linux runtime using OS-enforced namespaces, seccomp, SELinux and UID isolation
- [ ] Add resource limits, network/storage policies, process supervision and audit records
- [ ] Never let manifest metadata alone grant privileges

## Milestone 3 — Bootable device foundation
- [ ] Identify exact bootloader unlock state and vendor partition constraints
- [ ] Integrate an appropriate Linux kernel tree/config for SDM439
- [ ] Integrate device tree, display, touch, storage, USB, Wi-Fi/Bluetooth and modem drivers
- [ ] Establish verified boot/recovery and rollback-safe update chain
- [ ] Integrate vendor HAL/firmware where legally and technically available
- [ ] Build reproducible boot/vendor/recovery images with hashes and SBOM

## Milestone 4 — Mobile shell and services
- [ ] Implement Nila SystemUI and lock/home/notifications/quick settings/recents
- [ ] Implement Settings, Security Center, Vault and permission controls
- [ ] Implement installer with compatibility gate, backup, migration and rollback
- [ ] Implement external memory/zRAM policy with explicit storage constraints
- [ ] Implement accessibility, emergency calling and device-critical services before daily-use claims

## Milestone 5 — Validation
- [ ] Unit, integration, fuzz, dependency and static security scans
- [ ] Emulator/VM tests for services and UI where supported
- [ ] Boot/recovery/update test on a sacrificial test device with backups
- [ ] Hardware validation: display, touch, storage, Wi-Fi, Bluetooth, GNSS, audio, camera, modem and power
- [ ] Battery/memory/performance tests on the 3 GB target
- [ ] Document recovery path, known issues and supported build hashes

## Release gate
Do not call Nila OS bootable or production-ready until a reproducible image has been built, verified boot/recovery works, the actual target device has booted, core hardware is tested, and critical security tests pass. CI green is necessary but not sufficient.
