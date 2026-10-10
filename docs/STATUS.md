# Nila OS Status

## Implemented in v0.1.0 baseline
- Rust workspace
- MHR format detection
- SHA-256 file hashing
- MHF manifest generation
- MFR runtime selection
- MFR payload integrity verification
- Basic Nila security policy and validation
- Nila service registry and startup-policy validation
- MHR command-line tool
- vivo 1906 reference profile
- GitHub Actions Rust checks

## Added build-foundation work
- Python validator for the vivo 1906 device profile
- Guarded ARM64 kernel build helper for an externally supplied compatible kernel source tree
- CI checks for device metadata and kernel helper shell syntax
- Device integration requirements and a staged bootable-build plan

These additions validate configuration and provide a build entry point; they do not provide kernel source, vendor binaries, Android HALs, boot images, or hardware testing.

## Not yet implemented / verified
- Bootloader integration
- Verified SDM439 kernel source and exact PD1930F defconfig
- Qualcomm vendor blobs/HAL integration and firmware
- Device tree/DTBO and verified partition map
- Recovery environment and verified restore path
- Verified boot / AVB signing integration
- Full enforcing SELinux policy integrated with device services
- Android framework/vendor compatibility and product configuration
- Real MHF archive/package format and cryptographic package signatures
- MAR implementation
- Nila SystemUI runtime
- Installer runtime
- External Memory runtime
- Telephony/camera/audio/GNSS hardware integration
- Device-specific boot image packaging
- Bootable phone image and real-device boot/hardware tests

Do not describe this repository as a bootable Nila OS until a device-specific image has been built and its boot and hardware behavior have been tested and documented. See [BOOTABLE_BUILD.md](BOOTABLE_BUILD.md).
