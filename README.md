# Nila OS

Nila OS is a mobile-first operating-system engineering project. The initial device reference is the vivo Y11 (vivo 1906 / PD1930F, Qualcomm Snapdragon 439, ARM64). PC/Linux/VM environments are development and testing hosts only.

This repository is an engineering baseline. It is **not yet a bootable phone image**.

## Current baseline

- MHR file-format recognition and bounded-memory SHA-256 hashing
- MHF manifest metadata generation
- MFR runtime identification, payload integrity checking, and fail-closed execution authorization
- Nila security policy validation and startup service-list checks
- MHR command-line tool
- vivo 1906 reference profile and guarded kernel-build helper
- GitHub Actions checks for Rust code, device metadata, and build-helper input guards

## Safety and compatibility boundaries

- Recognizing an EXE, APP, APK, or SH extension does not translate or execute the application.
- No application runtime is implemented in this baseline. MFR refuses execution, even when manifest metadata looks valid.
- Package signature verification is not yet integrated into main; do not treat manifests as trusted installation artifacts.
- Rust policy checks and service lists are not kernel enforcement. SELinux, application sandboxing, verified boot, and device security require platform integration.
- Keep private signing keys out of this repository, command-line arguments, and device images.

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

The kernel helper requires an independently obtained kernel tree verified for this exact device and its exact defconfig. It does not download or invent missing vendor sources:

```bash
KERNEL_SRC=/absolute/path/to/verified/kernel \
KERNEL_DEFCONFIG=YOUR_VERIFIED_DEFCONFIG \
./scripts/build-kernel.sh
```

A successful Rust build or kernel compile does **not** prove phone bootability. See [Build Requirements](docs/BOOTABLE_BUILD.md), [Current Status](docs/STATUS.md), and [Roadmap](docs/ROADMAP.md).
