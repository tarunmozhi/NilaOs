package com.nila.assistant.core

data class PolicyDecision(
    val allowed: Boolean,
    val reason: String
)

object PolicyEngine {
    private val sensitivePatterns = listOf(
        Regex("(?i)\\b(password|passwd|passcode|otp|one[- ]time code|recovery code)\\b"),
        Regex("(?i)\\b(api key|access token|refresh token|secret key|private key)\\b"),
        Regex("(?i)\\bBearer\\s+[A-Za-z0-9._~+/=-]{8,}"),
        Regex("\\b[0-9]{12,19}\\b")
    )

    /**
     * Heuristic only: this is a conservative first guard, not a complete DLP engine.
     * Call before any user text is sent to a remote service.
     */
    fun evaluateOutboundText(text: String): PolicyDecision {
        if (text.isBlank()) {
            return PolicyDecision(false, "The query is empty.")
        }
        if (text.length > 500) {
            return PolicyDecision(false, "Research queries are limited to 500 characters.")
        }
        if (sensitivePatterns.any { it.containsMatchIn(text) }) {
            return PolicyDecision(false, "This text may contain private or secret information. It was not sent.")
        }
        return PolicyDecision(true, "Allowed by the local heuristic policy.")
    }
}
