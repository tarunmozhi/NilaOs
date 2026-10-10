# Build, Test, and Use the Nila OS Baseline

This guide applies to the Rust engineering baseline in this repository. It does not create a bootable Linux phone image.

## 1. Requirements

- Rust stable toolchain with Cargo
- Python 3.11 or newer
- Bash
- Git
- Network access for Cargo to download dependencies on first build

## 2. Get the repository

```bash
git clone https://github.com/tarunmozhi/NilaOs.git
cd NilaOs
```

If you already have a checkout, save or commit local changes before pulling updates.

## 3. Validate the workspace

Run all checks from the repository root:

```bash
python3 scripts/validate-device-profile.py
python3 -m unittest discover -s tests -p 'test_*.py'
bash -n scripts/build-kernel.sh
cargo fmt --all -- --check
cargo test --workspace
cargo build --workspace
cargo clippy --workspace -- -D warnings
```

GitHub Actions runs these checks on pull requests. Passing checks validate the Rust and configuration baseline only.

## 4. Analyze a file

```bash
cargo run -p mhr-cli -- analyze ./path/to/file.apk
```

This recognizes the filename extension and computes SHA-256. It does not inspect the full executable format or prove compatibility. APK recognition is metadata handling only; Nila OS remains Linux-based and no Android application runtime is implemented.

## 5. Create and verify manifest metadata

Create metadata:

```bash
cargo run -p mhr-cli -- manifest ./path/to/file.apk ExampleApp 0.1.0
```

This emits unsigned JSON metadata. After a trusted public key has been provisioned and a manifest signed by its corresponding private key, verify it with:

```bash
cargo run -p mhr-cli -- verify-manifest manifest.json config/nila-trusted-keys.toml
cargo run -p mhr-cli -- verify-package manifest.json ./payload.bin config/nila-trusted-keys.toml
```

These commands verify metadata and payload integrity. They do not install or execute the payload.

Keep private signing keys offline and out of this repository. An empty trusted-key store intentionally trusts no signers.

## 6. Check policy and startup configuration

```bash
cargo run -p mhr-cli -- check-policy
cargo run -p mhr-cli -- startup-check
```

These commands validate configuration and service entries. They do not enable Linux security modules in the host kernel, launch production services, or enforce app isolation.

## 7. Linux kernel build helper

The helper requires a compatible Linux kernel tree and the exact verified defconfig for the target device. It intentionally does not download a kernel or guess a defconfig:

```bash
KERNEL_SRC=/absolute/path/to/verified/kernel \
KERNEL_DEFCONFIG=YOUR_VERIFIED_DEFCONFIG \
./scripts/build-kernel.sh
```

A successful kernel compile does not produce a complete Nila OS image or prove that the phone can boot it. Native Linux userspace, firmware and drivers, device tree, partition compatibility, recovery, and boot-image packaging remain separate requirements.

## 8. Not supported yet

- Production signing-key creation, rotation, and revocation workflow
- Complete MHF archive/container and transactional installer
- Native Linux or optional Android app runtime execution
- Device-compatible SDM439 Linux kernel source and exact PD1930F defconfig
- Native Linux root filesystem, init/service manager, and graphical session
- Device firmware/drivers, recovery, verified boot, and enforcing Linux security integration
- Device-specific Linux image and real vivo 1906 boot/hardware tests

Treat passing Rust checks as validation of this code baseline only.
