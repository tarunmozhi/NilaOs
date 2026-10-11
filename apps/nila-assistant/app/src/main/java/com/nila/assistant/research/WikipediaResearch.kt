package com.nila.assistant.research

import com.nila.assistant.core.PolicyEngine
import org.json.JSONArray
import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL
import java.net.URLEncoder
import java.util.concurrent.Executors

data class ResearchResult(
    val title: String,
    val description: String,
    val url: String
)

object WikipediaResearch {
    private val executor = Executors.newSingleThreadExecutor()
    private const val MAX_RESULTS = 5
    private const val MAX_RESPONSE_BYTES = 512 * 1024

    fun search(query: String, callback: (String) -> Unit) {
        val decision = PolicyEngine.evaluateOutboundText(query)
        if (!decision.allowed) {
            callback(decision.reason)
            return
        }

        executor.execute {
            val resultText = try {
                val encoded = URLEncoder.encode(query, Charsets.UTF_8.name())
                val endpoint = URL(
                    "https://en.wikipedia.org/w/api.php?action=opensearch&search=" +
                        encoded + "&limit=" + MAX_RESULTS + "&namespace=0&format=json"
                )
                require(endpoint.protocol == "https" && endpoint.host == "en.wikipedia.org") {
                    "Research host rejected."
                }

                val connection = (endpoint.openConnection() as HttpURLConnection).apply {
                    requestMethod = "GET"
                    connectTimeout = 8000
                    readTimeout = 8000
                    setRequestProperty("User-Agent", "NilaAssistant/0.1 (Android research prototype)")
                }

                try {
                    if (connection.responseCode !in 200..299) {
                        error("Research service returned HTTP " + connection.responseCode)
                    }
                    val stream = connection.inputStream
                    val bytes = stream.use { input ->
                        val output = java.io.ByteArrayOutputStream()
                        val buffer = ByteArray(4096)
                        var total = 0
                        while (true) {
                            val count = input.read(buffer)
                            if (count < 0) break
                            total += count
                            require(total <= MAX_RESPONSE_BYTES) { "Research response exceeded size limit." }
                            output.write(buffer, 0, count)
                        }
                        output.toByteArray()
                    }
                    parse(JSONArray(String(bytes, Charsets.UTF_8)))
                } finally {
                    connection.disconnect()
                }
            } catch (error: Exception) {
                "Research failed: " + (error.message ?: "unknown error")
            }
            callback(resultText)
        }
    }

    private fun parse(data: JSONArray): String {
        if (data.length() < 4) return "No results were returned."
        val titles = data.optJSONArray(1) ?: return "No results were returned."
        val descriptions = data.optJSONArray(2) ?: JSONArray()
        val urls = data.optJSONArray(3) ?: JSONArray()
        if (titles.length() == 0) return "No results found."

        val lines = mutableListOf("Wikipedia search results:")
        for (index in 0 until minOf(titles.length(), MAX_RESULTS)) {
            val title = titles.optString(index)
            val description = descriptions.optString(index)
            val url = urls.optString(index)
            if (url.startsWith("https://en.wikipedia.org/")) {
                lines += (index + 1).toString() + ". " + title
                if (description.isNotBlank()) lines += description
                lines += url
                lines += ""
            }
        }
        return lines.joinToString("\n").ifBlank { "No safe result links were returned." }
    }
}
