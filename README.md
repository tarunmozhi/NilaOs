# Nila OS

Nila OS is a mobile-first operating-system engineering project targeting the vivo Y11 / vivo 1906 reference platform.

This repository is an engineering baseline. It is **not yet a bootable phone image**.

## Reference device
- vivo Y11 / vivo 1906
- PD1930F
- Qualcomm Snapdragon 439 / SDM439
- 8x ARM Cortex-A53
- 3 GB RAM
- 32 GB eMMC 5.1
- 720x1544 @ 60 Hz

## Core components
- MHR Core: file-format recognition, streaming SHA-256, and MHF manifest metadata
- MFR-Nila: payload integrity checks and Ed25519 manifest-signature verification
- Nila Security policy validation
- Nila service registry and startup validation
- MHR CLI
- vivo 1906 reference profile
- GitHub Actions CI

## Security behavior

- Unknown, missing, malformed, or untrusted package signatures are rejected by the signature-verification API.
- The trusted public-key store is `config/nila-trusted-keys.toml`. It is deliberately empty by default and therefore trusts no signer until an operator provisions a public key.
- Keep signing private keys offline. Never put private keys in this repository, the device image, or command-line arguments.
- `verify-package` verifies both the signed manifest and payload SHA-256 digest.
- A valid signature does **not** make an unsupported application safe or executable. MFR authorization still refuses execution because actual runtimes are not implemented.
- Rust policy checks and service lists are not kernel enforcement. SELinux, sandboxing, verified boot, and device security require platform integration.

## Important compatibility limitation

MHR recognizes EXE, APP, APK and SH filename extensions; recognition is not universal binary translation. APK support requires the Maha Android Runtime and Android/vendor compatibility. EXE/APP execution is not implemented by this baseline.

## Build and test

Run these commands from the repository root:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo build --workspace
cargo clippy --workspace -- -D warnings
```

Verify signed manifest metadata:

```bash
cargo run -p mhr-cli -- verify-manifest manifest.json config/nila-trusted-keys.toml
```

Verify signed metadata and the exact payload digest together:

```bash
cargo run -p mhr-cli -- verify-package manifest.json ./payload.bin config/nila-trusted-keys.toml
```

See [Build and Test](docs/BUILD_AND_TEST.md), [Current Status](docs/STATUS.md), and [Engineering Roadmap](docs/ROADMAP.md).

Passing Rust CI validates this code baseline only. It does not mean that a bootable image exists or that phone hardware has been tested.
