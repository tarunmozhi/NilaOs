package com.nila.assistant

import android.app.Activity
import android.app.AlertDialog
import android.os.Bundle
import android.view.ViewGroup
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.ScrollView
import android.widget.TextView
import com.nila.assistant.apps.AppActions
import com.nila.assistant.core.PolicyEngine
import com.nila.assistant.research.WikipediaResearch

class MainActivity : Activity() {
    private lateinit var prompt: EditText
    private lateinit var output: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val root = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(24, 24, 24, 24)
            layoutParams = ViewGroup.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT
            )
        }

        val heading = TextView(this).apply {
            text = "Nila Assistant"
            textSize = 26f
        }
        val subtitle = TextView(this).apply {
            text = "Local-first starter • தமிழ் / English"
            textSize = 14f
        }
        prompt = EditText(this).apply {
            hint = "Ask Nila or enter a research query"
            minLines = 2
            maxLines = 4
        }
        output = TextView(this).apply {
            text = "Vanakkam! I'm the Nila Assistant starter. I can answer a few local help prompts, search Wikipedia, and open Android's app-management screens. I am not a full AI model yet."
            textSize = 16f
            setTextIsSelectable(true)
        }

        root.addView(heading)
        root.addView(subtitle)
        root.addView(prompt)
        root.addView(makeButton("Ask Nila (local demo)") { answerLocally() })
        root.addView(makeButton("Research Wikipedia") { research() })
        root.addView(makeButton("Find app to install") { findApp() })
        root.addView(makeButton("Request app uninstall") { requestUninstall() })

        val scroll = ScrollView(this)
        scroll.addView(output)
        root.addView(
            scroll,
            LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                0,
                1f
            )
        )
        setContentView(root)
    }

    private fun makeButton(label: String, action: () -> Unit): Button {
        return Button(this).apply {
            text = label
            setOnClickListener { action() }
        }
    }

    private fun answerLocally() {
        val text = prompt.text.toString().trim()
        output.text = when {
            text.isBlank() -> "Type a question first."
            text.contains("help", ignoreCase = true) ||
                text.contains("உதவி") ->
                "I can currently: answer this basic help prompt, search Wikipedia, open the Play Store search, and request Android's uninstall confirmation. Full AI, voice, memory and video generation are not connected yet."
            text.contains("வணக்கம்") ->
                "வணக்கம்! சொல்லுங்க மாமா — இந்த prototype-ல் இன்னும் முழு AI இணைக்கப்படவில்லை."
            else ->
                "Local demo received your message. No cloud AI was called. Add a local model or a reviewed inference adapter to enable real conversational answers."
        }
    }

    private fun research() {
        val query = prompt.text.toString().trim()
        val decision = PolicyEngine.evaluateOutboundText(query)
        if (!decision.allowed) {
            output.text = decision.reason
            return
        }
        output.text = "Searching Wikipedia…"
        WikipediaResearch.search(query) { result ->
            runOnUiThread { output.text = result }
        }
    }

    private fun findApp() {
        val field = EditText(this).apply { hint = "App name, e.g. VLC" }
        AlertDialog.Builder(this)
            .setTitle("Search app store")
            .setMessage("Nila will open the store/search page. You review the app and choose whether to install it.")
            .setView(field)
            .setNegativeButton("Cancel", null)
            .setPositiveButton("Open search") { _, _ ->
                val opened = AppActions.searchForApp(this, field.text.toString())
                output.text = if (opened) {
                    "Store search opened. Review the app, developer and permissions before installing."
                } else {
                    "No compatible store or browser was found."
                }
            }
            .show()
    }

    private fun requestUninstall() {
        val field = EditText(this).apply {
            hint = "Package ID, e.g. org.videolan.vlc"
        }
        AlertDialog.Builder(this)
            .setTitle("Request app uninstall")
            .setMessage("Enter the exact package ID. Android will show its confirmation screen; Nila cannot silently uninstall it.")
            .setView(field)
            .setNegativeButton("Cancel", null)
            .setPositiveButton("Continue") { _, _ ->
                val packageName = field.text.toString()
                val opened = AppActions.requestUninstall(this, packageName)
                output.text = if (opened) {
                    "Android's uninstall screen was requested. Review the package and confirm or cancel there."
                } else {
                    "Invalid package ID or Android could not open the uninstall screen."
                }
            }
            .show()
    }
}
