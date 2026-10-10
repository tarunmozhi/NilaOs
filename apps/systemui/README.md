# Nila System UI

Nila Skin is the visual shell planned for Nila OS's native Linux mobile user session. This directory currently contains a browser prototype only; it is not yet a native Linux UI or a bootable OS component.

## Nila Skin prototype

Open [nila-skin/index.html](nila-skin/index.html) in a browser to preview the Home screen, Lock screen, Control Center, App drawer, Notifications, Settings, Performance, External Memory, Security & Privacy, Vault, Recents, Search, customization, first-run setup, Power menu, and Recovery.

The preview uses sample data and preview-only actions. It does not require Android SystemUI, AOSP, or Android services. See [the prototype README](nila-skin/README.md) for its scope and Linux integration direction.

A native implementation must route privileged operations through permission-checked Nila services; UI event handlers must not perform privileged or destructive operations directly.
