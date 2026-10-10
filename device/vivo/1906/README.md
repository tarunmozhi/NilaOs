# vivo 1906 Linux device integration

Target: vivo Y11 / model PD1930F, Qualcomm Snapdragon 439 (SDM439), ARM64.

Nila OS targets a Linux kernel and native Linux userspace. The adjacent `device.toml` records hardware and stock-firmware references; it is not a complete Linux device configuration. The phone's Android/Funtouch firmware can help identify its boot chain, partition layout, kernel lineage, firmware, and available driver sources. It is not Nila's operating-system base.

Before a device image can be built, this directory needs a verified Linux kernel/device-tree configuration, boot and partition definitions, Linux mount and service-startup configuration, hardware drivers and firmware, recovery, and references to compatible components. Do not assume all vivo Y11 regional variants share identical partition layouts, firmware, or bootloader behavior.

Verify binary provenance and licensing. Do not substitute another SDM439 device's files without validation, guess a partition map, or flash unverified images to a daily-use phone.

See [the Linux boot plan](../../../docs/BOOTABLE_BUILD.md).
