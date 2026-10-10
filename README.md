# Nila OS

Nila OS is a mobile-first **Linux-based operating system project** targeting the vivo Y11 / vivo 1906 (PD1930F), Qualcomm Snapdragon 439 / SDM439, ARM64.

Nila's intended base is the Linux kernel and a native Linux userspace. Nila OS is not Android or an AOSP distribution. Android/Funtouch details in the device profile describe the phone's stock firmware and hardware reference only. Android app compatibility is a possible optional runtime; it does not define or provide the Nila OS base.

PC, Linux, and virtual machines are development and test hosts only. This repository is an engineering baseline, **not a bootable phone image**. It currently lacks the device kernel source, native root filesystem, boot services, hardware integration, and image packaging needed to boot Nila.

See [Target Architecture](docs/ARCHITECTURE.md), [Current Status](docs/STATUS.md), and the [Roadmap](docs/ROADMAP.md).

## Current code baseline

- MHR file-format recognition and bounded-memory SHA-256 hashing
- MHF manifest metadata generation with optional detached Ed25519 signature metadata
- MFR trusted-key signature verification and payload digest checking
- Fail-closed execution authorization while application runtimes are absent
- Nila security policy validation and startup service-list checks
- MHR command-line tool
- vivo 1906 reference profile and guarded Linux kernel-build helper
- GitHub Actions checks for Rust code, device metadata, and build-helper input guards

These are engineering foundations. They do not yet constitute a Linux distribution or a mobile operating system.

## Compatibility and security boundaries

- Recognizing an EXE, APP, APK, or SH extension does not translate or execute the application.
- No application runtime is implemented. In particular, the Linux runtime and optional Android compatibility runtime are not implemented; MFR refuses execution even when manifest metadata has a valid trusted signature.
- The trusted public-key store is `config/nila-trusted-keys.toml`. It is intentionally empty by default, so no signer is trusted until a public key is provisioned.
- The repository does not implement a production signing command, key rotation/revocation, or a release-key lifecycle. Keep signing private keys offline; never commit them.
- Rust policy checks and service lists are not kernel enforcement. Linux security modules, app isolation, verified boot, and device security require platform integration.

## Build and test

Run from the repository root:

```bash
python3 scripts/validate-device-profile.py
python3 -m unittest discover -s tests -p 'test_*.py'
bash -n scripts/build-kernel.sh
cargo fmt --all -- --check
cargo test --workspace
cargo build --workspace
cargo clippy --workspace -- -D warnings
```

These checks validate the current code baseline; they do not build Nila's Linux kernel or root filesystem.

The kernel helper requires an independently obtained kernel tree verified for this exact device and its exact defconfig. It does not download or invent missing vendor sources:

```bash
KERNEL_SRC=/absolute/path/to/verified/kernel \
KERNEL_DEFCONFIG=YOUR_VERIFIED_DEFCONFIG \
./scripts/build-kernel.sh
```

A successful Rust build or kernel compile does **not** prove phone bootability. See [Linux Boot Requirements](docs/BOOTABLE_BUILD.md).
