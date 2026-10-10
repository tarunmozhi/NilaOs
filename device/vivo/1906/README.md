# vivo 1906 device integration

Target: vivo Y11 / model PD1930F, Qualcomm Snapdragon 439 (SDM439), ARM64.

The adjacent device.toml is descriptive metadata, not a complete device tree. Before a device image can be built, this directory needs a verified device-specific product configuration, init scripts, fstab, VINTF manifest/matrices, SELinux rules, partition definitions, display/touch configuration, and references to compatible kernel/vendor components.

Do not assume all vivo Y11 regional variants share identical partition layouts, firmware, or bootloader behavior. Record the exact source firmware build and verify all binary provenance and licensing. Never use a guessed partition map or flash unverified images to a daily-use phone.

See docs/BOOTABLE_BUILD.md for the build plan.
