# Nila OS Status

## Implemented in the current code baseline
- Rust workspace and CI checks
- MHR extension-based format recognition
- Streaming SHA-256 hashing with bounded memory use
- MHF manifest metadata generation
- MFR runtime identification and payload integrity verification
- Fail-closed execution authorization while application runtimes are absent
- Basic Nila security policy validation
- Nila service registry and startup-policy validation
- MHR command-line tool
- vivo 1906 reference profile with schema validation and regression tests
- Guarded ARM64 kernel-build helper with defconfig and job-count validation

## Important security limits
- Package signature verification is not integrated into main; manifests are not trusted installation artifacts.
- The runtime names are identifiers only. No APK/MAR, Linux, native, EXE, or APP execution is implemented.
- Policy validation and service-list construction do not enforce SELinux or create operating-system services.
- Hash verification checks payload bytes against a supplied digest; it does not establish who supplied the digest or whether the payload is safe.

## Not yet implemented / verified
- Bootloader integration
- Verified SDM439 kernel source and exact PD1930F defconfig
- Qualcomm vendor blobs/HAL integration and firmware
- Device tree/DTBO and verified partition map
- Recovery environment and verified restore path
- Verified boot / AVB signing integration
- Full enforcing SELinux policy integrated with device services
- Android framework/vendor compatibility and product configuration
- Real MHF archive/package format and cryptographic package-signature enforcement
- Maha Android Runtime (MAR)
- Nila SystemUI runtime
- Installer runtime
- External Memory runtime
- Telephony/camera/audio/GNSS hardware integration
- Device-specific boot image packaging
- Bootable phone image and real-device boot/hardware tests

These additions validate configuration and provide a guarded build entry point; they do not provide kernel source, vendor binaries, Android HALs, boot images, or hardware testing.

Do not describe this repository as a bootable Nila OS until a device-specific image has been built and its boot and hardware behavior have been tested and documented. See [BOOTABLE_BUILD.md](BOOTABLE_BUILD.md).
