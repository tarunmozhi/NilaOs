# Nila SystemUI

Nila's mobile SystemUI implementation area.

## Nila Skin

An interactive full-screen UI prototype based on the shared **Nila OS Dark 1** reference lives in [`nila-skin/`](nila-skin/). Open [`nila-skin/index.html`](nila-skin/index.html) in a browser to preview Home, Lock screen, Control Center, App drawer, Notifications, Settings, Performance, External Memory, Security & Privacy, Vault, Recents, Search, customization, first-run setup, Power menu, and Recovery.

The prototype uses sample data and preview-only actions. It is not yet a native Android SystemUI or bootable OS component. See [the prototype README](nila-skin/README.md) for integration boundaries and next steps.

Native integration must route privileged actions through permission-checked Nila services; UI controls must not directly execute destructive or privileged operations.
