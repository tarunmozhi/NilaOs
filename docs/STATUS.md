# Nila OS Status

## Implemented in v0.1.0 engineering baseline
- Rust workspace and GitHub Actions CI
- MHR filename-extension recognition and streaming SHA-256 hashing
- MHF manifest metadata with detached signature metadata
- Ed25519 manifest-signature verification against an explicit trusted-key map
- Trusted public-key TOML loading and validation
- MFR payload integrity verification
- Fail-closed MFR authorization for missing/untrusted signatures, unsupported apps, and absent runtimes
- Basic Nila security policy validation
- Nila service registry and startup configuration validation
- MHR CLI
- vivo 1906 reference profile

## Not yet implemented
- A production release-signing workflow and secure key provisioning/rotation/revocation process
- A complete MHF archive/package format and package installer
- Payload-to-manifest installation transaction and protection against file replacement between verification and use
- MAR implementation and Linux/native runtime execution
- Bootloader integration and device-specific boot image generation
- SDM439 kernel source/configuration integration
- Qualcomm vendor blobs/HAL integration
- Recovery environment and verified boot chain
- Full SELinux policy and kernel-enforced app sandbox
- Android framework/vendor compatibility
- Nila SystemUI runtime
- Installer runtime
- External Memory runtime
- Telephony/camera/audio/GNSS hardware integration
- Emulator and physical vivo 1906 boot/testing

## Security boundary

Ed25519 verification authenticates manifest metadata against public keys explicitly supplied by the caller. `config/nila-trusted-keys.toml` is intentionally empty by default; with no provisioned trusted key, no signer is trusted. This repository does not yet contain a production private key, key rotation service, installer, or executable runtime. The supported manifest flag is untrusted metadata and cannot bypass runtime checks.

The current startup check validates configuration and returns a service registry; it does not launch services or enforce SELinux. Do not describe the repository as a bootable Nila OS until device-specific kernel, boot/recovery, vendor, security policy, image generation, and hardware testing are complete.
