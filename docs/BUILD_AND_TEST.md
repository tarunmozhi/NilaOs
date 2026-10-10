# Build, Test, and Use Nila OS Baseline

This guide applies to the Rust engineering baseline in this repository. It does not create a bootable Android/phone image.

## 1. Requirements
- Rust stable toolchain with Cargo
- Git
- Network access for Cargo to download dependencies on first build

## 2. Get the review branch

```bash
git clone https://github.com/tarunmozhi/NilaOs.git
cd NilaOs
git fetch origin
git checkout review/streaming-hash-and-integrity
```

On Windows PowerShell, the same Git commands work if Git is installed.

## 3. Validate the workspace

Run all checks from the repository root:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo build --workspace
cargo clippy --workspace -- -D warnings
```

A command succeeds when it exits with status code 0. CI executes the same checks on pushes and pull requests.

## 4. Analyze a file

```bash
cargo run -p mhr-cli -- analyze ./path/to/file.apk
```

This recognizes the filename extension and computes SHA-256. It does not inspect the full executable format or prove compatibility. Unsupported formats are reported as unsupported.

## 5. Create manifest metadata

```bash
cargo run -p mhr-cli -- manifest ./path/to/file.apk ExampleApp 0.1.0
```

This emits unsigned JSON metadata. The trusted-key configuration is intentionally empty and the repository does not yet include a production signing command. Unsigned metadata cannot pass signature verification.

## 6. Verify a signed manifest

After a trusted public key has been provisioned and a manifest signed by its corresponding private key, run:

```bash
cargo run -p mhr-cli -- verify-manifest manifest.json config/nila-trusted-keys.toml
```

This verifies manifest metadata only.

## 7. Verify a signed manifest and payload together

```bash
cargo run -p mhr-cli -- verify-package manifest.json ./payload.bin config/nila-trusted-keys.toml
```

The command checks the manifest signature and hashes the payload against the signed SHA-256 digest. It does not install or execute the payload.

## 8. Check security policy and startup configuration

```bash
cargo run -p mhr-cli -- check-policy
cargo run -p mhr-cli -- startup-check
```

These commands validate configuration and startup service entries. They do not enable SELinux in the host kernel, launch production services, or enforce app isolation.

## 9. Windows local save folder

To keep a local checkout under the intended Nila project folder:

```powershell
git clone https://github.com/tarunmozhi/NilaOs.git E:\Arunmozhi\Nila
cd E:\Arunmozhi\Nila
git fetch origin
git checkout review/streaming-hash-and-integrity
```

If the folder already contains a Git repository, do not clone over it. Instead run `git fetch origin` and `git checkout review/streaming-hash-and-integrity` inside the existing checkout.

## 10. Not supported yet
- No production signing-key creation/rotation/revocation workflow
- No complete MHF archive or transactional installer
- No runtime execution for MAR, Linux, EXE, APP, or native packages
- No device kernel/boot/recovery image build
- No vivo 1906 emulator or physical-device boot test

Treat passing Rust checks as validation of this code baseline only.
