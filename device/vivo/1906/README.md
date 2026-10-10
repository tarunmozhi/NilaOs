# vivo 1906 Linux device integration

## Device identity

The Nila profile targets vivo Y11 / vivo 1906, Qualcomm Snapdragon 439 (SDM439), ARM64. The profile lists model reference `PD1930F`; the user-confirmed stock build string is `PD1930CF_EX_A_6.71.15`. These identifiers must remain distinct until the exact phone/product and board identifiers establish their mapping. Vivo's [public Y11 update page](https://www.vivo.com/in/support/upgradePackageData?id=82) lists a PD1930F package; that does not alone verify the user's CF build variant.

## Candidate kernel source

A local source checkout supplied for this project was inspected read-only. Its Git remote points to [vivo-source-mirror/Y11](https://github.com/vivo-source-mirror/Y11), commit `9c04dc87d779a852c62cbd2605a725fc0295b99f`. The checkout reports Linux 4.9.227 and includes:

- `msm8937_defconfig`, which enables `CONFIG_ARCH_SDM439=y`
- `sdm439-mtp-overlay-PD1930F_EX.dts`, with board ID `<8 24>`
- `sdm439-mtp-overlay-PD1930BF_EX.dts`, with board ID `<8 25>`

The tree does not name an overlay `PD1930CF`. The source's build note uses an `aarch64-linux-android-4.9` cross compiler and outputs `Image.gz-dtb`. This makes the checkout a candidate source for the Y11 family; it does not prove it is the exact source/configuration for the CF build, and the mirror's provenance has not been independently authenticated.

## Remaining device work

Before selecting an overlay or packaging an image, verify the phone's exact product/model/board identifiers, stock boot and partition layout, kernel and DT/DTBO selection, modules, and required firmware. Do not infer that the PD1930F overlay applies to PD1930CF or PD1930BF from the shared Y11 name or SDM439 chipset.

The GitHub repository does not include this source tree, a native Linux root filesystem, early userspace, an init/service manager, boot/recovery packaging, or a tested restore path. No kernel build or phone boot has been performed for Nila OS.

See [the Linux boot plan](../../../docs/BOOTABLE_BUILD.md) and the [current project status](../../../docs/STATUS.md).
