# Nila OS Status

## Project target

Nila OS is a mobile Linux operating-system project: Linux kernel plus native Linux userspace, with a Nila graphical shell. It is not Android or AOSP. The vivo device profile's Android/Funtouch values refer to the phone's stock firmware only.

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

These code checks and registries do not provide a running kernel, userspace, init system, or service enforcement.

## Important security limits

- The trusted-key store is empty by default; no signer is trusted until a public key is provisioned.
- Production signing-key generation, rotation, revocation, and release signing are not implemented.
- The runtime names are identifiers only. No native Linux, Android compatibility, EXE, or APP execution is implemented.
- Policy validation and service-list construction do not enforce Linux security modules or create operating-system services.
- Hash verification checks payload bytes against a supplied digest; it does not establish who supplied the digest or whether the payload is safe.

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
- Nila Skin as a native Linux graphical shell and system UI
- Installer runtime and External Memory runtime
- Telephony/camera/audio/GNSS hardware integration
- Device-specific Linux boot and root filesystem image packaging
- Bootable phone image and real-device boot/hardware tests

Device firmware or vendor components may be needed to support specific hardware, but an Android framework is not part of the Nila OS base. These additions validate configuration and provide a guarded kernel build entry point; they do not provide a kernel source tree, Linux userspace, boot image, or hardware testing.

Do not describe this repository as a bootable Nila OS until a device-specific Linux image has been built and its boot and hardware behavior have been tested and documented. See [BOOTABLE_BUILD.md](BOOTABLE_BUILD.md).
