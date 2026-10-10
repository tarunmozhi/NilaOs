# SDM439 kernel integration

This repository directory is only a placeholder; it does not contain the kernel source tree or a compiled kernel.

A user-provided candidate Y11 source checkout was inspected outside this repository. It reports Linux 4.9.227 and identifies the Git remote `vivo-source-mirror/Y11` at commit `9c04dc87d779a852c62cbd2605a725fc0295b99f`. It contains `msm8937_defconfig` with SDM439 enabled and DTBO files for `PD1930F_EX` and `PD1930BF_EX`, but no overlay explicitly named `PD1930CF`. The remote mirror's Vivo provenance and the exact mapping to the reported `PD1930CF_EX_A_6.71.15` build are not verified.

The candidate's build note expects an `aarch64-linux-android-4.9` toolchain and produces `Image.gz-dtb`. The Nila helper supports an explicit `CROSS_COMPILE` prefix and writes output outside the source tree. A kernel compile alone would not provide a root filesystem or a bootable Nila image.

Do not copy the external tree into this directory or select a DTBO until source provenance, exact device/board ID, boot chain, and partition requirements are verified. See [device integration notes](../../device/vivo/1906/README.md) and the [Linux boot plan](../../docs/BOOTABLE_BUILD.md).
