package com.nila.assistant.core

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class PolicyEngineTest {
    @Test
    fun blocksPasswordLikeQueries() {
        assertFalse(PolicyEngine.evaluateOutboundText("my password is abc123").allowed)
    }

    @Test
    fun blocksBearerTokens() {
        assertFalse(
            PolicyEngine.evaluateOutboundText("Bearer abcdefghijklmnop").allowed
        )
    }

    @Test
    fun blocksLongNumericIdentifiers() {
        assertFalse(PolicyEngine.evaluateOutboundText("check 1234567890123456").allowed)
    }

    @Test
    fun allowsOrdinaryResearchQuery() {
        assertTrue(PolicyEngine.evaluateOutboundText("What is Snapdragon 439?").allowed)
    }

    @Test
    fun blocksEmptyAndOversizedQueries() {
        assertFalse(PolicyEngine.evaluateOutboundText("").allowed)
        assertFalse(PolicyEngine.evaluateOutboundText("x".repeat(501)).allowed)
    }
}
