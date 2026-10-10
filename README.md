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
- Nila service registry and bootstrap validation
- MHR CLI
- vivo 1906 reference profile
- GitHub Actions CI checks

## Build and test existing components

On a host with Rust stable and Python 3.11 or newer:

    python3 scripts/validate-device-profile.py
    bash -n scripts/build-kernel.sh
    cargo fmt --all -- --check
    cargo test --workspace
    cargo build --workspace
    cargo clippy --workspace -- -D warnings

The kernel helper requires an independently obtained, device-compatible Linux kernel tree and its verified defconfig. It does not download a kernel or package a phone image.

## Bootable device build

See [docs/BOOTABLE_BUILD.md](docs/BOOTABLE_BUILD.md) for the staged engineering plan and required device-specific inputs. See [device/vivo/1906/README.md](device/vivo/1906/README.md) for hardware integration requirements.

## Compatibility limitation

MHR recognizes EXE, APP, APK and SH formats, but recognition is not universal binary translation. APK support depends on the Maha Android Runtime and available Android/vendor compatibility. EXE/APP execution is not implemented by this baseline.

Do not install or flash a build until device-specific boot images, recovery, partition compatibility, integrity checks, and a recovery path have been verified.
