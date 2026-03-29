use crate::HeroCodeBlock;
use leptix_ui::password_toggle::*;
use leptos::prelude::*;

#[component]
pub fn PasswordTogglePage() -> impl IntoView {
    view! {
        <h1>"Password Toggle"</h1>
        <p class="description">
            "A form field that allows users to toggle the visibility of their password input."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="password-toggle">
            <div class="hero-demo-card">
            <div class="demo-password-toggle">
                <PasswordToggleField>
                    <PasswordToggleFieldInput />
                    <PasswordToggleFieldToggle>"Show"</PasswordToggleFieldToggle>
                </PasswordToggleField>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_ui::password_toggle::*;\n\nview! {\n    <PasswordToggleField attr:class=\"PasswordToggleFieldRoot\">\n        <PasswordToggleFieldInput attr:class=\"PasswordToggleFieldInput\" />\n        <PasswordToggleFieldToggle attr:class=\"PasswordToggleFieldToggle\">\"Show\"</PasswordToggleFieldToggle>\n    </PasswordToggleField>\n}"
            css_styles=".PasswordToggleFieldRoot {\n  display: flex;\n  align-items: center;\n  gap: 8px;\n}\n\n.PasswordToggleFieldInput {\n  flex: 1;\n  padding: 8px 12px;\n  border-radius: 4px;\n  border: 1px solid var(--black-a9);\n  background: transparent;\n  color: white;\n  font-size: 15px;\n  outline: none;\n}\n\n.PasswordToggleFieldInput:focus {\n  border-color: var(--violet-9);\n}\n\n.PasswordToggleFieldToggle {\n  padding: 8px 12px;\n  border-radius: 4px;\n  border: none;\n  background-color: var(--violet-9);\n  color: white;\n  font-size: 13px;\n  font-weight: 500;\n  cursor: pointer;\n}\n\n.PasswordToggleFieldToggle:hover {\n  background-color: var(--violet-10);\n}"
            modules_usage="use leptix_ui::password_toggle::*;\n// use styles from module\n\nview! {\n    <PasswordToggleField attr:class=styles.root>\n        <PasswordToggleFieldInput attr:class=styles.input />\n        <PasswordToggleFieldToggle attr:class=styles.toggle>\"Show\"</PasswordToggleFieldToggle>\n    </PasswordToggleField>\n}"
            modules_styles=".root {\n  display: flex;\n  align-items: center;\n  gap: 8px;\n}\n\n.input {\n  flex: 1;\n  padding: 8px 12px;\n  border-radius: 4px;\n  border: 1px solid var(--black-a9);\n  background: transparent;\n  color: white;\n  font-size: 15px;\n  outline: none;\n}\n\n.input:focus {\n  border-color: var(--violet-9);\n}\n\n.toggle {\n  padding: 8px 12px;\n  border-radius: 4px;\n  border: none;\n  background-color: var(--violet-9);\n  color: white;\n  font-size: 13px;\n  font-weight: 500;\n  cursor: pointer;\n}\n\n.toggle:hover {\n  background-color: var(--violet-10);\n}"
            tailwind_usage="use leptix_ui::password_toggle::*;\n\nview! {\n    <PasswordToggleField attr:class=\"flex items-center gap-2\">\n        <PasswordToggleFieldInput\n            attr:class=\"flex-1 py-2 px-3 rounded border border-gray-600 bg-transparent text-white text-sm outline-none focus:border-violet-500\" />\n        <PasswordToggleFieldToggle\n            attr:class=\"py-2 px-3 rounded bg-violet-500 text-white text-xs font-medium cursor-pointer hover:bg-violet-600\">\n            \"Show\"\n        </PasswordToggleFieldToggle>\n    </PasswordToggleField>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {},\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Toggle password visibility with a button."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Accessible label and toggle button."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Fully unstyled and composable."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-password-toggle"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_password_toggle::*;\n\nview! {\n    <PasswordToggleField>\n        <PasswordToggleFieldInput />\n        <PasswordToggleFieldToggle />\n    </PasswordToggleField>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="field">"Field"</h3>
        <p>"Contains all the parts of the password toggle field."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"visible"</td><td>"MaybeProp<bool>"</td><td>"The controlled visibility state."</td></tr>
                    <tr><td>"default_visible"</td><td>"MaybeProp<bool>"</td><td>"The default visibility state when initially rendered. Default: false."</td></tr>
                    <tr><td>"on_visible_change"</td><td>"Callback<bool>"</td><td>"Event handler called when visibility changes."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="input">"Input"</h3>
        <p>"The password input element. Automatically switches between "<code>"type=\"password\""</code>" and "<code>"type=\"text\""</code>" based on visibility state."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"visible\" | \"hidden\""</td></tr>
            </tbody>
        </table>

        <h3 id="toggle">"Toggle"</h3>
        <p>"The button that toggles the visibility of the password."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"visible\" | \"hidden\""</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"The toggle button is properly labeled for screen readers and communicates the current visibility state."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"When focus is on the toggle, toggles the password visibility."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"When focus is on the toggle, toggles the password visibility."</td></tr>
            </tbody>
        </table>
    }
}
