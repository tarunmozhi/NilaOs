# Nila Assistant for Android

Standalone Android application prototype, independent of the Nila OS boot/runtime work. Intended to be installable as an ordinary APK on Android 11+ and later reusable as the Nila OS assistant UI.

## Current prototype scope

- Bilingual Tamil/English chat interface.
- Local-only deterministic fallback so the app can be tested without a network or API key.
- Explicit online-AI opt-in setting placeholder.
- No root permissions and no device-control tools.
- Never embed provider API keys in the APK.
- Does not yet contain a language model. A model adapter and secure backend/proxy must be added before it can provide ChatGPT-like generated answers.

## Build

Open this directory in a recent Android Studio and allow Gradle sync. Use a compatible installed Android SDK. Build the debug APK using Android Studio's Build > Build APK(s), or run:

```bash
gradle assembleDebug
```

A generated APK is not checked into source control. Test on an emulator first, then on the vivo 1906. The app has not yet been built or device-tested by this repository change.
