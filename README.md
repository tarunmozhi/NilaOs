# Nila OS

Nila OS is a mobile-first operating-system engineering project. The initial device reference is the vivo Y11 (vivo 1906 / PD1930F, Qualcomm Snapdragon 439, ARM64). PC/Linux/VM environments are development and testing hosts only.

This repository is an engineering baseline. It is **not yet a bootable phone image**.

## Current baseline

- MHR file-format recognition and bounded-memory SHA-256 hashing
- MHF manifest metadata generation with optional detached Ed25519 signature metadata
- MFR trusted-key signature verification and payload digest checking
- Fail-closed execution authorization while application runtimes are absent
- Nila security policy validation and startup service-list checks
- MHR command-line tool
- vivo 1906 reference profile and guarded kernel-build helper
- GitHub Actions checks for Rust code, device metadata, and build-helper input guards

## Safety and compatibility boundaries

- Recognizing an EXE, APP, APK, or SH extension does not translate or execute the application.
- No application runtime is implemented in this baseline. MFR refuses execution, even when manifest metadata has a valid trusted signature.
- The trusted public-key store is `config/nila-trusted-keys.toml`. It is intentionally empty by default, so no signer is trusted until a public key is provisioned.
- The repository does not implement a production signing command, key rotation/revocation, or a release-key lifecycle. Keep signing private keys offline; never commit them.
- Rust policy checks and service lists are not kernel enforcement. SELinux, application sandboxing, verified boot, and device security require platform integration.

## Run checks on a Windows PC

A guarded PowerShell runner is available at `scripts/run-nila-checks.ps1`. It clones or safely fast-forwards this repository into `E:\\Arunmozhi\\Nila`, validates the device profile, runs Python/Rust checks, and checks the kernel helper's shell syntax. It refuses to overwrite a dirty checkout.

Open PowerShell and run:

```powershell
Set-ExecutionPolicy -Scope Process Bypass
& "E:\\Arunmozhi\\Nila\\scripts\\run-nila-checks.ps1"
```

Requirements: Git, Python 3.11+, and the stable Rust toolchain. The script never flashes a phone. Kernel compilation is skipped unless you explicitly provide both a verified kernel source tree and its exact device defconfig; on Windows it uses WSL and the helper still requires the AArch64 cross-compiler.

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

Create unsigned manifest metadata:

```bash
cargo run -p mhr-cli -- manifest ./payload.apk ExampleApp 0.1.0
```

Verify a signed manifest against explicitly trusted public keys:

```bash
cargo run -p mhr-cli -- verify-manifest manifest.json config/nila-trusted-keys.toml
```

Verify the signed manifest and exact payload bytes together:

```bash
cargo run -p mhr-cli -- verify-package manifest.json ./payload.bin config/nila-trusted-keys.toml
```

These commands verify metadata and payload integrity; they do not install or execute the application.

The kernel helper requires an independently obtained kernel tree verified for this exact device and its exact defconfig. It does not download or invent missing vendor sources:

```bash
KERNEL_SRC=/absolute/path/to/verified/kernel \
KERNEL_DEFCONFIG=YOUR_VERIFIED_DEFCONFIG \
./scripts/build-kernel.sh
```

A successful Rust build or kernel compile does **not** prove phone bootability. See [Build Requirements](docs/BOOTABLE_BUILD.md), [Current Status](docs/STATUS.md), and [Roadmap](docs/ROADMAP.md).
