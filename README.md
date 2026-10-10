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
- Signature verification authenticates signed manifest metadata; payload integrity must also be checked against the manifest digest.
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

Verify a signed manifest's metadata with the CLI:

```bash
cargo run -p mhr-cli -- verify-manifest path/to/manifest.json config/nila-trusted-keys.toml
```

A successful signature check is not an installation or runtime test. See `docs/STATUS.md` for the implementation boundary and remaining work.
