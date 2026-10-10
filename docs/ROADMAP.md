# Nila OS Engineering Roadmap

## Target architecture

Nila OS is a mobile-only, Linux-based operating-system project. The intended system is a Linux kernel with a native Linux userspace and Nila mobile shell; it is not Android or an AOSP distribution. The vivo Y11 / vivo 1906 (PD1930F), Qualcomm Snapdragon 439 / SDM439, ARM64, 3 GB RAM, 32 GB eMMC, and 720×1544 display are the initial hardware target. Its installed Android/Funtouch firmware is a device reference only. PC/Linux/VM environments are development and test hosts.

The base system milestone below remains the foundation. After it, the project targets Nila Skin and Easy Touch, app runtimes, Nila Browser, Nila Assistant, security controls, external memory, installation, updates, and recovery. See [Target Architecture](ARCHITECTURE.md) for the full design and current implementation limits. Target features are not completed features.

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

## Milestone 2 — MHF packages, runtimes, and app isolation

- [ ] Define a versioned MHF container format, canonical manifest encoding, payload layout, and permission schema
- [ ] Implement a streaming package reader with strict size limits and archive path traversal protection
- [ ] Add safe staging, fsync, atomic commit/rollback, and TOCTOU protection
- [ ] Add malformed-container fuzz tests and property tests
- [ ] Define release signing, key rotation, revocation, and recovery procedures
- [ ] Implement an approved native Linux application runtime with strict per-app sandboxing
- [ ] Run .sh files only through a restricted Linux environment with explicit permissions
- [ ] Keep Android APK compatibility as a separate optional runtime; it is not part of the Linux boot or base userland
- [ ] Define and implement EXE compatibility separately; recognizing .exe must never imply execution support
- [ ] Resolve whether .app means an Apple application bundle or a Nila package before selecting its runtime
- [ ] Enforce separate app identities, capability grants, storage/network policies, and resource limits before enabling app execution
- [ ] Provide non-root sandbox profiles for banking, RailOne, and similar apps; test each app's compatibility and provider integrity checks

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

## Milestone 4 — Nila Skin, system UI, browser, and installation

- [ ] Implement Nila Skin as the shell and system UI for Nila's Linux graphical session
- [ ] Build the launcher, quick settings, notifications, dialogs, widgets, and gesture framework
- [ ] Integrate Easy Touch as user-toggleable, movable, and customizable shortcuts; keep preview state distinct from system actions
- [ ] Implement theme packs with accurate attribution and accessible low-memory settings
- [ ] Implement Nila Browser as a native app; evaluate the Chromium engine, extension compatibility, privacy controls, and optional VPN/Tor integration
- [ ] Connect UI and browser actions to permission-checked native Nila services
- [ ] Implement a graphical and PC/USB-assisted installer with compatibility checks, backup, verification, migration, and rollback
- [ ] Package a device-specific Linux boot and root filesystem image only after kernel, recovery, partition, and integrity checks are verified

## Milestone 5 — Security, owner profile, and systemless administration

- [ ] Define one interactive owner profile while retaining separate service and per-app identities
- [ ] Implement the selected enforcing Linux security policy, permission broker, app capability grants, and audit service
- [ ] Design optional user-controlled systemless administrator access; keep root unavailable to apps and Assistant processes
- [ ] Add per-app consent controls that users can review and revoke
- [ ] Verify isolation between Assistant, browser, native apps, and financial-app sandboxes
- [ ] Test locked/unlocked boot states and document which security guarantees the device can actually provide

## Milestone 6 — Nila Assistant

- [ ] Implement an internet-connected, voice-first AI service as an unprivileged process
- [ ] Add user-controlled permissions per app, account, and capability, including clear disable/revoke controls
- [ ] Implement a credential/session broker so account secrets are not exposed to the model or general logs
- [ ] Add permission-checked web/app operation and local tools
- [ ] Integrate call handling and approved voice/audio controls through native telephony APIs
- [ ] Require explicit confirmation for financial, security-sensitive, and other consequential actions
- [ ] Test that Assistant actions cannot escape app sandboxes or obtain root

## Milestone 7 — Storage, memory, updates, and recovery

- [ ] Implement Nila Storage and External Memory services with off/phone/SD/automatic/custom selection
- [ ] Add encryption, integrity, safe removal, access control, and flash-wear safeguards
- [ ] Evaluate zRAM and optional storage-backed swap on the actual device before enabling either by default
- [ ] Implement signed update delivery, compatibility gates, atomic installation, recovery, and rollback
- [ ] Test power-loss recovery and verify updates on a recoverable test device before release

## Release gates

A passing Rust build is not a bootable operating system. Do not flash an image until all device-specific components, partition compatibility, verified boot, recovery, and rollback have been validated. A release requires a recorded boot test and hardware test on the actual supported device; no such phone test is currently claimed.
