# Nila OS Status

## Project target

Nila OS is a mobile Linux operating-system project: Linux kernel plus native Linux userspace, with a Nila graphical shell. It is not Android or AOSP. The vivo device profile's Android/Funtouch values refer to the phone's stock firmware only.

The product target beyond the base system includes Nila Skin and Easy Touch, a native Nila Browser, MHF/MHR application handling, an internet-connected Nila Assistant, security and app-isolation services, External Memory, an installer, updates, and recovery. The target architecture and implementation boundaries are described in [ARCHITECTURE.md](ARCHITECTURE.md) and tracked in [ROADMAP.md](ROADMAP.md).

Nila Assistant is intended to search online and operate apps and account sessions when enabled by the user. It should support call handling and user-selected voice features through permission-checked system APIs. The user should control access by app, account, and capability. Banking, RailOne, and similar apps must remain in separate non-root sandboxes; Assistant access is opt-in and cannot bypass those boundaries.

The current repository is an engineering baseline, not a Linux distribution or bootable phone image.

## Implemented in the current code baseline

- Rust workspace and CI checks
- MHR extension-based format recognition
- Streaming SHA-256 hashing with bounded memory use
- MHF manifest metadata generation
- MFR runtime identification and payload integrity verification
- Fail-closed execution authorization while application runtimes are absent
- Ed25519 verification for signed MHF manifest metadata against an explicitly provisioned trusted-key store
- Combined CLI verification of signed manifest metadata and exact payload SHA-256
- Basic Nila security policy validation
- Nila service registry and startup-policy validation
- MHR command-line tool
- vivo 1906 reference profile with schema validation and regression tests
- Guarded ARM64 Linux kernel-build helper with defconfig and job-count validation
- Nila Skin browser preview with user-toggleable, draggable, customizable Easy Touch shortcuts

These code checks, registries, and preview do not provide a running kernel, userspace, init system, service enforcement, or native phone UI.

## Target feature status

| Component | Current state |
| --- | --- |
| Nila UI framework and Nila Skin | Browser prototype only; no native Linux shell or system UI |
| Easy Touch | Toggle, move, and shortcut customization work in the browser preview; native system actions are not connected |
| Nila Browser | Not implemented as an application; no selected or integrated web engine, extension service, privacy controller, VPN, or Tor integration |
| Nila Assistant | Unprivileged service registry entry and documented requirements only; no AI runtime, internet access, account/session broker, call control, voice processing, or app permission UI |
| MHF/MHR application support | Extension recognition, metadata, hashing, signature checks, and fail-closed authorization; no container installer or executable runtime |
| .sh, .apk, .exe, and .app execution | Not implemented. The current .app mapping means Apple application metadata; package meaning must be defined before implementation |
| Per-app security and financial-app sandboxes | Not implemented or enforced by the kernel; banking and RailOne apps have not been tested |
| Single owner profile and systemless administration | Not implemented; no root manager or privilege broker |
| Nila Security Center, Vault, memory/storage/network/power/package/update services | Names and policy checks only where present; no complete operating-system services |
| External Memory, zRAM, storage-backed swap | Not implemented or benchmarked on the target device |
| Installer, PC/USB-assisted installation, signed OTA, recovery, rollback | Not implemented |
| Themes | Preview customization only; no native theme service or verified third-party theme integration |

## Important security limits

- The trusted-key store is empty by default; no signer is trusted until a public key is provisioned.
- Production signing-key generation, rotation, revocation, and release signing are not implemented.
- The runtime names are identifiers only. No native Linux, Android compatibility, EXE, or APP execution is implemented.
- Policy validation and service-list construction do not enforce Linux security modules or create operating-system services.
- Hash verification checks payload bytes against a supplied digest; it does not establish who supplied the digest or whether the payload is safe.
- Banking or RailOne compatibility cannot be promised until an isolated runtime is implemented and each app is tested with its provider's integrity requirements.

## Not yet implemented / verified

- Verified vivo 1906 Linux kernel source and exact PD1930F configuration
- Native Linux root filesystem, early userspace, init/service manager, and boot-time services
- Device tree/DTBO, kernel modules, firmware, and verified partition map
- Linux device drivers and hardware integration
- Recovery environment and verified restore path
- Verified boot integration for the device's actual boot chain
- Full enforcing Linux security policy integrated with device services
- Real MHF archive/package format and transactional installer (signature verification currently covers manifest metadata plus a signed payload digest, not archive parsing)
- Native Linux application runtime and optional Maha Android Runtime (MAR)
- Separate enforced non-root app sandboxes, including profiles for banking and RailOne apps
- User-controlled systemless root mechanism that keeps root unavailable to apps and assistant processes
- Nila Skin as a native Linux graphical shell and system UI
- Native Nila Browser and its privacy, extension, VPN, and Tor integrations
- Nila Assistant runtime, internet access, voice interaction, app/account permission controls, protected credential/session broker, call control, and approved voice features
- Installer runtime, External Memory runtime, zRAM, and storage-backed swap
- Telephony/camera/audio/GNSS hardware integration
- Device-specific Linux boot and root filesystem image packaging
- Bootable phone image and real-device boot/hardware tests

Device firmware or vendor components may be needed to support specific hardware, but an Android framework is not part of the Nila OS base. These additions validate configuration and provide a guarded kernel build entry point; they do not provide a kernel source tree, Linux userspace, boot image, or hardware testing.

Do not describe this repository as a bootable Nila OS until a device-specific Linux image has been built and its boot and hardware behavior have been tested and documented. See [BOOTABLE_BUILD.md](BOOTABLE_BUILD.md).
