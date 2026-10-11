package org.nila.assistant;

import android.app.Activity;
import android.os.Bundle;
import android.graphics.Color;
import android.view.Gravity;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

/**
 * Standalone Nila Assistant MVP shell.
 * The deterministic reply is a connectivity/UI placeholder, not an LLM.
 */
public final class MainActivity extends Activity {
    private LinearLayout messages;
    private EditText input;
    private boolean onlineEnabled = false;

    @Override
    public void onCreate(Bundle state) {
        super.onCreate(state);
        getWindow().getDecorView().setLayoutDirection(ViewGroup.LAYOUT_DIRECTION_LOCALE);
        getWindow().setStatusBarColor(Color.rgb(13, 35, 58));
        getWindow().setNavigationBarColor(Color.rgb(13, 35, 58));

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(dp(16), dp(16), dp(16), dp(12));
        root.setBackgroundColor(Color.rgb(9, 20, 34));

        TextView title = new TextView(this);
        title.setText("Nila Assistant  •  நிலா");
        title.setTextSize(23);
        title.setTextColor(Color.rgb(117, 220, 255));
        title.setGravity(Gravity.CENTER_VERTICAL);
        root.addView(title, new LinearLayout.LayoutParams(-1, dp(52)));

        TextView subtitle = new TextView(this);
        subtitle.setText("Tamil + English  |  Local-first prototype");
        subtitle.setTextColor(Color.LTGRAY);
        root.addView(subtitle);

        Button online = new Button(this);
        online.setText("Online AI: OFF (not connected)");
        online.setOnClickListener(v -> {
            onlineEnabled = !onlineEnabled;
            online.setText(onlineEnabled
                ? "Online AI: ON (provider not configured)"
                : "Online AI: OFF (not connected)");
            addMessage("Nila", onlineEnabled
                ? "Online mode selected, but no provider is connected yet."
                : "Offline mode selected. Your messages stay on this device in this prototype.");
        });
        root.addView(online);

        ScrollView scroll = new ScrollView(this);
        messages = new LinearLayout(this);
        messages.setOrientation(LinearLayout.VERTICAL);
        messages.setPadding(0, dp(8), 0, dp(8));
        scroll.addView(messages);
        root.addView(scroll, new LinearLayout.LayoutParams(-1, 0, 1f));

        LinearLayout composer = new LinearLayout(this);
        composer.setOrientation(LinearLayout.HORIZONTAL);
        input = new EditText(this);
        input.setSingleLine(false);
        input.setMinLines(1);
        input.setMaxLines(4);
        input.setHint("Ask in Tamil or English…");
        input.setTextColor(Color.WHITE);
        input.setHintTextColor(Color.GRAY);
        input.setBackgroundTintList(android.content.res.ColorStateList.valueOf(
            Color.rgb(117, 220, 255)));
        composer.addView(input, new LinearLayout.LayoutParams(0, -2, 1f));

        Button send = new Button(this);
        send.setText("Send");
        send.setOnClickListener(v -> sendMessage());
        composer.addView(send);
        root.addView(composer);

        setContentView(root);
        addMessage("Nila", "வணக்கம்! I'm Nila. Ask me in Tamil or English.\n\nThis is the starter app; connect a real AI model to enable full answers.");
    }

    private void sendMessage() {
        String text = input.getText().toString().trim();
        if (text.isEmpty()) return;
        input.setText("");
        addMessage("You", text);
        boolean tamil = text.codePoints().anyMatch(cp -> cp >= 0x0B80 && cp <= 0x0BFF);
        String reply = tamil
            ? "உங்கள் கேள்வியைப் பெற்றேன்: " + text + "\nமுழுமையான AI பதில்களுக்கு local model இணைப்பு தேவை."
            : "I received: " + text + "\nA real local or approved online model must be connected for a full answer.";
        addMessage("Nila", reply);
    }

    private void addMessage(String who, String text) {
        TextView bubble = new TextView(this);
        bubble.setText(who + "\n" + text);
        bubble.setTextSize(16);
        bubble.setTextColor(Color.WHITE);
        bubble.setPadding(dp(12), dp(10), dp(12), dp(10));
        bubble.setBackgroundColor(who.equals("You")
            ? Color.rgb(31, 65, 88) : Color.rgb(24, 42, 58));
        LinearLayout.LayoutParams lp = new LinearLayout.LayoutParams(-1, -2);
        lp.setMargins(0, dp(5), 0, dp(5));
        messages.addView(bubble, lp);
    }

    private int dp(int value) {
        return (int) (value * getResources().getDisplayMetrics().density + 0.5f);
    }
}
