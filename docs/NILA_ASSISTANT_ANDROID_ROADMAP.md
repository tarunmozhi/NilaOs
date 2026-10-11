# Nila Assistant standalone Android roadmap

## Goal

Ship Nila Assistant as an independent, portable Android APK first. Nila OS can later integrate or reuse compatible components. Do not make the app depend on Nila OS boot, kernel, MFR-Nila, or root access.

## Milestones

1. **MVP shell:** Android chat UI, Tamil/English input, deterministic offline placeholder, no dangerous permissions.
2. **Local inference:** integrate a maintained on-device inference runtime and a small quantized multilingual model. Benchmark peak RAM, first-token latency, tokens/sec, storage, heat, and battery on emulator and vivo 1906. Do not assume a 3 GB device can run a particular model until measured.
3. **Conversation quality:** bounded conversation context, clear/reset history, optional local SQLite history, user-controlled saved memories, and refusal/uncertainty behavior.
4. **Online provider:** opt-in provider interface, TLS, backend proxy for provider credentials, timeouts, response-size limits, redaction, and a visible notice of what is transmitted. Never ship API secrets in the APK.
5. **Knowledge retrieval:** local documents indexed with source attribution; only public/approved material may be sent online. Treat retrieved documents and tool output as untrusted data, not policy instructions.
6. **Voice:** Android speech recognition and text-to-speech with explicit microphone permission; provide a text-only fallback. Tamil voice availability depends on installed Android speech services.
7. **Training:** curate bilingual Tamil/English examples; keep training, validation, and test sets separate; deduplicate; remove personal secrets; record data provenance and licenses. Start with prompting/RAG. Fine-tune a suitable pretrained model only when evaluations show a measurable need.
8. **Nila OS integration:** package shared Rust policy/core logic behind a stable interface if appropriate. Keep Android UI and provider adapters separate from MHR Core and the MFR-Nila app runtime.

## Security invariants

- App runs without root; never request accessibility, SMS, contacts, call logs, or broad storage access merely for chat.
- Private and secret zones are local-only by default; online routing rejects them unless a future explicit, narrowly scoped, user-confirmed policy is designed and tested. Secret data should remain local.
- AI text is never executable code or authorization. Tools use typed allowlisted requests and a separate permission broker.
- Consequential actions require confirmation; logs exclude secrets and provider credentials.
- Online mode is off by default; network access should be added only alongside a real, reviewed provider implementation.

## Acceptance tests

- App launches and chat UI works in airplane mode.
- Tamil and English messages both render without corruption.
- Empty messages are ignored.
- Online mode defaults off and does not imply a connected provider.
- No root permissions or privileged service is requested.
- Local/private/secret data routing tests fail closed.
- API keys are absent from source and APK.
- A real APK build, emulator smoke test, and vivo 1906 test are recorded separately. No build or device success is claimed until actually run.
