# Build, Test, and Use Nila OS Baseline

This guide applies to the Rust engineering baseline in this repository. It does not create a bootable Android/phone image.

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

GitHub Actions runs these checks on pull requests. Passing checks validate the Rust/configuration baseline only.

## 4. Analyze a file

```bash
cargo run -p mhr-cli -- analyze ./path/to/file.apk
```

This recognizes the filename extension and computes SHA-256. It does not inspect the full executable format or prove compatibility.

## 5. Create manifest metadata

```bash
cargo run -p mhr-cli -- manifest ./path/to/file.apk ExampleApp 0.1.0
```

This emits unsigned JSON metadata. The trusted-key configuration is intentionally empty and the repository does not include a production signing command. Unsigned metadata cannot pass signature verification.

## 6. Verify a signed manifest

After a trusted public key has been provisioned and a manifest signed by its corresponding private key, run:

```bash
cargo run -p mhr-cli -- verify-manifest manifest.json config/nila-trusted-keys.toml
```

This verifies manifest metadata only. It does not verify the payload bytes.

## 7. Verify a signed manifest and payload together

```bash
cargo run -p mhr-cli -- verify-package manifest.json ./payload.bin config/nila-trusted-keys.toml
```

The command checks the manifest signature and hashes the payload against the signed SHA-256 digest. It does not install or execute the payload.

The trusted-key file accepts public Ed25519 keys only. Keep private signing keys offline and out of this repository. An empty trusted-key store intentionally trusts no signers.

## 8. Check security policy and startup configuration

```bash
cargo run -p mhr-cli -- check-policy
cargo run -p mhr-cli -- startup-check
```

These commands validate configuration and startup service entries. They do not enable SELinux in the host kernel, launch production services, or enforce app isolation.

## 9. Kernel build helper

The helper requires a compatible Linux kernel tree and the exact verified defconfig for the target device. It intentionally does not download a kernel or guess a defconfig:

```bash
KERNEL_SRC=/absolute/path/to/verified/kernel \
KERNEL_DEFCONFIG=YOUR_VERIFIED_DEFCONFIG \
./scripts/build-kernel.sh
```

A successful kernel compile does not prove that the phone can boot it. Vendor modules, firmware, device tree, partition compatibility, AVB, recovery, and boot-image packaging must be validated separately.

## 10. Not supported yet
- Production signing-key creation, rotation, and revocation workflow
- Complete MHF archive/container and transactional installer
- Runtime execution for MAR, Linux, EXE, APP, or native packages
- Device-compatible SDM439 kernel source and exact PD1930F defconfig
- Qualcomm vendor blobs/HALs, device tree, recovery, AVB, and enforcing SELinux integration
- Device-specific boot image and real vivo 1906 boot/hardware tests

Treat passing Rust checks as validation of this code baseline only.
