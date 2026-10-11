package com.nila.assistant.apps

import android.app.Activity
import android.content.ActivityNotFoundException
import android.content.Intent
import android.net.Uri

object AppActions {
    fun searchForApp(activity: Activity, appName: String): Boolean {
        val name = appName.trim().take(120)
        if (name.isBlank()) return false

        val storeIntent = Intent(
            Intent.ACTION_VIEW,
            Uri.parse("market://search?q=" + Uri.encode(name))
        )
        return try {
            activity.startActivity(storeIntent)
            true
        } catch (_: ActivityNotFoundException) {
            val webIntent = Intent(
                Intent.ACTION_VIEW,
                Uri.parse("https://play.google.com/store/search?q=" + Uri.encode(name))
            )
            try {
                activity.startActivity(webIntent)
                true
            } catch (_: ActivityNotFoundException) {
                false
            }
        }
    }

    /**
     * Opens Android's confirmation flow. The user must confirm the uninstall.
     * This function never attempts silent package removal.
     */
    fun requestUninstall(activity: Activity, packageName: String): Boolean {
        val name = packageName.trim()
        if (!name.matches(Regex("[A-Za-z0-9_]+(\\.[A-Za-z0-9_]+)+"))) return false

        val intent = Intent(Intent.ACTION_DELETE, Uri.parse("package:" + name))
        return try {
            activity.startActivity(intent)
            true
        } catch (_: ActivityNotFoundException) {
            false
        }
    }
}
