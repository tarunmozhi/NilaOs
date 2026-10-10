# Nila Skin — Linux UI prototype

This folder contains a self-contained interactive browser prototype for the planned Nila Skin mobile shell. Nila OS targets a Linux kernel and native Linux userspace; this preview is a design prototype, not an Android SystemUI or a native Linux application.

It follows the shared **Nila OS Dark 1** reference: deep navy surfaces, electric-blue controls, cyan focus accents, rounded cards, compact icon grids, and reduced-effects options for a low-memory mobile target.

## Preview

Open `index.html` directly in a desktop or mobile browser. No build tools, network requests, dependencies, or install permissions are required.

## Screens included

- Lock screen and unlock-to-home preview
- Home screen, app dock, app drawer, and app search
- Control Center with interactive quick-setting tiles and brightness slider
- Notifications with All / Unread / Silent filters and clear action
- Settings navigation and filtering
- Edge-peeking Easy Touch button, enabled by default in the preview but user-toggleable; users can customize app and settings shortcuts and drag it anywhere on screen, with keyboard arrow movement supported
- Performance metrics and power-mode selector (sample values only)
- External Memory choices: Off, Phone Storage, SD Card, Automatic, Custom
- Security & Privacy, Nila Vault, and guarded recovery / power menus
- Recents, global search, first-run setup, and Nila Skin customization

## Implementation boundary

All controls change preview state only. Easy Touch's Settings toggle and shortcut selections are not connected to native services. Memory choices do not configure zRAM or storage; security and Vault controls are not connected to privileged services; power and recovery actions are deliberately non-destructive. Metrics and notifications are sample data.

The native implementation belongs in Nila's Linux user session and must use the Linux display, input, audio, and service stack selected for the device. It must not depend on Android SystemUI or AOSP. Android application compatibility, if added later, is a separate optional runtime and does not define the Nila OS base. Route privileged operations through permission-checked Nila services.

## Local checks

The prototype is plain HTML/CSS/JavaScript and can be opened directly. There is no JavaScript build step. Check layout at narrow phone widths and desktop widths; verify keyboard focus, screen-reader labels, and reduced-motion behavior before shipping.
