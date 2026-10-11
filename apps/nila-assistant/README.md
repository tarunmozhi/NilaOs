# Nila Assistant — Android starter

This is a lightweight Android app foundation for Nila Assistant. It is separate from the Nila OS Linux userspace and does not yet integrate with Nila OS services.

## Included

- Minimal local help response (not an LLM).
- Wikipedia search API over HTTPS with conservative outbound-query screening.
- Search for an app in the Play Store.
- Request Android's system uninstall confirmation for a user-supplied package ID.
- A small policy helper and unit tests for obvious secret-like text.
- Media policy scaffolding only; no image/video generation implementation.

## Build

Requirements: JDK 17, Android SDK Platform 35, Android build tools, and Gradle 8.9.

From this directory, if Gradle is installed:

    gradle assembleDebug
    gradle testDebugUnitTest

Alternatively, open this directory in Android Studio, sync the Gradle project, and run the app on an emulator or Android device. If your environment requires a Gradle wrapper, generate it with Gradle 8.9 using:

    gradle wrapper --gradle-version 8.9

The app requires internet permission for Wikipedia research. Research results are displayed with links; no general web crawler is included. The app does not silently install or uninstall apps. Those actions are handed to the store or Android system UI.

## Important limitations

- Not a ChatGPT-equivalent assistant; no LLM is bundled.
- No persistent chat memory, wake-word listener, voice pipeline, browser crawler, arbitrary code execution, root shell, or remote AI provider is implemented.
- No video-generation provider is connected.
- The secret filter uses simple patterns and can miss secrets or flag ordinary text. Do not treat it as a complete DLP system.
- The app has not been compiled or device-tested as part of this commit. CI/build results must be checked before treating it as validated.
