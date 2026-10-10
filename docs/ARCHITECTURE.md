# Nila OS target architecture

## Project identity

Nila OS is intended to be a Linux-based mobile operating system. Its base will be a Linux kernel plus Nila's native Linux userspace. It is not based on Android or AOSP.

The vivo 1906 profile records the phone's factory firmware as a device reference. That information helps identify the hardware, boot chain, firmware, and available driver sources; it does not make Android the Nila OS base.

## Intended layers

1. **Device boot and Linux kernel.** Use a verified kernel source, device configuration, device tree, and boot path for the exact vivo 1906 variant.
2. **Native Linux userspace.** Provide a device root filesystem, early userspace, initialization and service management, storage and networking setup, and Nila's system services.
3. **Nila mobile shell.** Run the Nila Skin and system controls in a Linux graphical session, connected to permission-checked Nila services.
4. **Applications.** Support native Linux applications as the primary target. Android APK compatibility may be developed as an optional, isolated runtime; it is not required to boot or operate the Linux base.

## Current implementation boundary

The repository currently contains Rust engineering tools and metadata, not these complete system layers. The guarded kernel helper can build an externally supplied kernel tree when its exact device source and defconfig are provided. It does not supply the kernel, root filesystem, init system, graphical session, or phone image.

No Linux distribution, C library, init/service manager, compositor, package format, or Android compatibility implementation has been selected or completed here. Do not infer one from the current metadata or runtime labels.
