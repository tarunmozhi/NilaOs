# Nila OS target architecture

## Project identity

Nila OS is intended to be a Linux-based mobile operating system. Its base will be a Linux kernel plus Nila's native Linux userspace. It is not based on Android or AOSP.

The vivo 1906 profile records the phone's factory firmware as a device reference. That information helps identify the hardware, boot chain, firmware, and available driver sources; it does not make Android the Nila OS base.

## Intended layers

1. **Device boot and Linux kernel.** Use a verified kernel source, device configuration, device tree, and boot path for the exact vivo 1906 variant.
2. **Native Linux userspace.** Provide a device root filesystem, early userspace, initialization and service management, storage and networking setup, and Nila's system services.
3. **Nila mobile shell.** Run the Nila Skin and system controls in a Linux graphical session, connected to permission-checked Nila services.
4. **Applications.** Support native Linux applications as the primary target. Android APK compatibility may be developed as an optional, isolated runtime; it is not required to boot or operate the Linux base.
5. **Nila Assistant.** Provide an internet-connected, voice-first assistant that can search online, operate apps and account sessions, manage calls, and use user-selected voice features when the user enables the relevant capabilities.

## Nila Assistant permissions and account safety

The Assistant's intended power comes from user-granted capabilities, not from running as root. Users must be able to enable or disable access by app, account, and capability, review what is enabled, and revoke it.

Account sign-in and session use should go through an OS credential/session broker and isolated app or browser contexts. Stored passwords and authentication secrets must not be exposed to the assistant model or copied into general logs. The Assistant can act in an account only after the user grants that access. Actions with financial, security, or other consequential effects must receive an explicit user confirmation at the point of action.

The Assistant remains an unprivileged service. Phone calls, voice/audio changes, app control, and system settings must use permission-checked OS APIs and produce an audit trail. Banking, RailOne, and similar apps must run in separate non-root sandboxes. Assistant access to one of those apps is available only when the user enables it; that access must not give the app or Assistant root or bypass the app sandbox.

These are product and security requirements, not implemented capabilities in the current repository.

## Current implementation boundary

The repository currently contains Rust engineering tools and metadata, not these complete system layers. The guarded kernel helper can build an externally supplied kernel tree when its exact device source and defconfig are provided. It does not supply the kernel, root filesystem, init system, graphical session, or phone image.

No Linux distribution, C library, init/service manager, compositor, package format, or Android compatibility implementation has been selected or completed here. Do not infer one from the current metadata or runtime labels.
