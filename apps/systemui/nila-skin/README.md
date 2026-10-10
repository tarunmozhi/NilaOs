# Nila Skin — Full UI prototype

This folder contains a self-contained, interactive browser prototype for the Nila Skin visual direction. It follows the shared **Nila OS Dark 1** reference: deep navy surfaces, electric-blue controls, cyan focus accents, rounded cards, compact icon grids, and reduced-effects options suitable for a low-memory mobile target.

## Preview

Open `index.html` directly in a desktop or mobile browser. No build tools, network requests, dependencies, or install permissions are required.

## Screens included

- Lock screen and unlock-to-home preview
- Home screen, app dock, app drawer, and app search
- Control Center with interactive quick-setting tiles and brightness slider
- Notifications with All / Unread / Silent filters and clear action
- Settings navigation and filtering
- Performance metrics and power-mode selector (sample values only)
- External Memory choices: Off, Phone Storage, SD Card, Automatic, Custom
- Security & Privacy, Nila Vault, and guarded recovery / power menus
- Recents, global search, first-run setup, and Nila Skin customization

## Important implementation boundary

This is a **UI prototype**, not a bootable SystemUI or a native Android application. Interactive controls change preview state only. Memory choices do not configure zRAM or storage; security and Vault controls are not connected to privileged services; power and recovery actions are deliberately non-destructive. Metrics and notifications are sample data.

The next native integration step is to port the screen structure and design tokens into the chosen Android/AOSP SystemUI implementation, then connect actions through permission-checked Nila services. Do not expose privileged system operations directly to UI event handlers. Validate on an emulator first, then on the supported vivo 1906 device only after a compatible boot chain and kernel are available.

## Local checks

The prototype is plain HTML/CSS/JavaScript and can be opened directly. There is no JavaScript build step. Check layout at narrow phone widths and desktop widths; verify keyboard focus, screen-reader labels, and reduced-motion behavior before shipping.
