package com.nila.assistant.media

data class MediaRequest(
    val prompt: String,
    val involvesMinor: Boolean = false,
    val depictsRealPersonInIntimateContext: Boolean = false,
    val consentConfirmed: Boolean = false
)

data class MediaPolicyDecision(
    val allowedForProviderReview: Boolean,
    val reason: String
)

/**
 * Policy gate only. No image/video model or provider is connected in this starter.
 * A provider must perform its own policy checks before any generation.
 */
object MediaRequestPolicy {
    fun evaluate(request: MediaRequest): MediaPolicyDecision {
        if (request.prompt.isBlank()) {
            return MediaPolicyDecision(false, "A prompt is required.")
        }
        if (request.involvesMinor) {
            return MediaPolicyDecision(false, "Requests involving sexualization of minors are prohibited.")
        }
        if (request.depictsRealPersonInIntimateContext && !request.consentConfirmed) {
            return MediaPolicyDecision(false, "Intimate imagery of a real person requires verified consent and provider approval.")
        }
        return MediaPolicyDecision(
            true,
            "This request may proceed to a compliant provider's own review; generation is not implemented here."
        )
    }
}
