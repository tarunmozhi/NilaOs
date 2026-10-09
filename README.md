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
- MHR Core / MHR Runtime
- MHF package metadata
- MFR-Nila runtime selector and integrity checks
- Nila Security policy
- Nila service registry
- MHR CLI
- vivo 1906 device profile
- CI tests

## Important limitation
MHR recognizes EXE, APP, APK and SH formats, but recognition is not universal binary translation. APK support depends on the Maha Android Runtime and available Android/vendor compatibility. EXE/APP execution is not implemented by this baseline.

## Build and test

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo build --workspace
```

See `docs/STATUS.md` for the current implementation status.
