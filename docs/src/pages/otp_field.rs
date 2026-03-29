use crate::HeroCodeBlock;
use leptix_ui::otp_field::*;
use leptos::prelude::*;

#[component]
pub fn OtpFieldPage() -> impl IntoView {
    view! {
        <h1>"OTP Field"</h1>
        <p class="description">
            "A one-time password input with individual character slots."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="otp-field">
            <div class="hero-demo-card">
            <div class="demo-otp-field">
                <OneTimePasswordField>
                    <OneTimePasswordFieldInput index=0usize />
                    <OneTimePasswordFieldInput index=1usize />
                    <OneTimePasswordFieldInput index=2usize />
                    <OneTimePasswordFieldInput index=3usize />
                    <OneTimePasswordFieldInput index=4usize />
                    <OneTimePasswordFieldInput index=5usize />
                    <OneTimePasswordFieldHiddenInput />
                </OneTimePasswordField>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            usage_code="use leptix_ui::otp_field::*;\n\nview! {\n    <OneTimePasswordField>\n        <OneTimePasswordFieldInput index=0usize />\n        <OneTimePasswordFieldInput index=1usize />\n        <OneTimePasswordFieldInput index=2usize />\n        <OneTimePasswordFieldInput index=3usize />\n        <OneTimePasswordFieldInput index=4usize />\n        <OneTimePasswordFieldInput index=5usize />\n        <OneTimePasswordFieldHiddenInput />\n    </OneTimePasswordField>\n}"
            css_code=".OneTimePasswordFieldRoot {\n  display: flex;\n  gap: 8px;\n}\n\n.OneTimePasswordFieldInput {\n  width: 40px;\n  height: 48px;\n  text-align: center;\n  font-size: 20px;\n  font-weight: 600;\n  border-radius: 6px;\n  border: 2px solid var(--black-a9);\n  background: transparent;\n  color: white;\n  outline: none;\n}\n\n.OneTimePasswordFieldInput:focus {\n  border-color: var(--violet-9);\n  box-shadow: 0 0 0 1px var(--violet-9);\n}"
            css_modules_code=".Root {\n  display: flex;\n  gap: 8px;\n}\n\n.Input {\n  width: 40px;\n  height: 48px;\n  text-align: center;\n  font-size: 20px;\n  font-weight: 600;\n  border-radius: 6px;\n  border: 2px solid var(--black-a9);\n  background: transparent;\n  color: white;\n  outline: none;\n}\n\n.Input:focus {\n  border-color: var(--violet-9);\n  box-shadow: 0 0 0 1px var(--violet-9);\n}"
            tailwind_code="view! {\n    <OneTimePasswordField class=\"flex gap-2\">\n        <OneTimePasswordFieldInput index=0usize\n            class=\"w-10 h-12 text-center text-xl font-semibold rounded-md border-2 border-gray-600 bg-transparent text-white outline-none focus:border-violet-500 focus:ring-1 focus:ring-violet-500\" />\n        <OneTimePasswordFieldInput index=1usize\n            class=\"w-10 h-12 text-center text-xl font-semibold rounded-md border-2 border-gray-600 bg-transparent text-white outline-none focus:border-violet-500 focus:ring-1 focus:ring-violet-500\" />\n        // ... more slots\n        <OneTimePasswordFieldHiddenInput />\n    </OneTimePasswordField>\n}"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Automatically focuses the next input on entry."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports paste from clipboard across all slots."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Hidden input for form submission."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-otp-field"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_otp_field::*;\n\nview! {\n    <OneTimePasswordField>\n        <OneTimePasswordFieldInput index=0usize />\n        <OneTimePasswordFieldInput index=1usize />\n        // ... more slots\n        <OneTimePasswordFieldHiddenInput />\n    </OneTimePasswordField>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of the OTP field."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The controlled value."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<String>"</td><td>"Event handler called when the value changes."</td></tr>
                    <tr><td>"on_complete"</td><td>"Callback<String>"</td><td>"Event handler called when all slots are filled."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents interaction."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="input">"Input"</h3>
        <p>"A single character input slot."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"index"</td><td>"usize"</td><td>"The position of this slot in the OTP sequence."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="hidden-input">"HiddenInput"</h3>
        <p>"A hidden input that holds the combined OTP value for form submission."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Each input slot is labelled and focus management is automatic."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Moves focus to the next slot."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Moves focus to the previous slot."</td></tr>
                <tr><td><kbd>"Backspace"</kbd></td><td>"Clears the current slot and moves focus to the previous slot."</td></tr>
            </tbody>
        </table>
    }
}
