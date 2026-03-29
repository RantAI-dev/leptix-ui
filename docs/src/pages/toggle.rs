use crate::HeroCodeBlock;
use leptix_ui::toggle::*;
use leptos::prelude::*;

#[component]
pub fn TogglePage() -> impl IntoView {
    view! {
        <h1>"Toggle"</h1>
        <p class="description">
            "A two-state button that can be either on or off."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="toggle">
            <div class="hero-demo-card">
            <Toggle attr:class="demo-toggle" default_pressed=false>
                <span style="font-size:16px;font-weight:700">"B"</span>
            </Toggle>
            </div>
        </div>
        <HeroCodeBlock
            usage_code="use leptix_ui::toggle::*;\n\nview! {\n    <Toggle default_pressed=false>\n        <span style=\"font-weight:700\">\"B\"</span>\n    </Toggle>\n}"
            css_code=".ToggleRoot {\n  all: unset;\n  background-color: white;\n  color: var(--mauve-11);\n  height: 35px;\n  width: 35px;\n  border-radius: 4px;\n  display: flex;\n  font-size: 15px;\n  line-height: 1;\n  align-items: center;\n  justify-content: center;\n  box-shadow: 0 2px 10px var(--black-a7);\n  cursor: pointer;\n}\n.ToggleRoot:hover {\n  background-color: var(--violet-3);\n}\n.ToggleRoot[data-state=\"on\"] {\n  background-color: var(--violet-6);\n  color: var(--violet-12);\n}"
            css_modules_code=".Root {\n  all: unset;\n  background-color: white;\n  color: var(--mauve-11);\n  height: 35px;\n  width: 35px;\n  border-radius: 4px;\n  display: flex;\n  font-size: 15px;\n  line-height: 1;\n  align-items: center;\n  justify-content: center;\n  box-shadow: 0 2px 10px var(--black-a7);\n  cursor: pointer;\n}\n.Root:hover {\n  background-color: var(--violet-3);\n}\n.Root[data-state=\"on\"] {\n  background-color: var(--violet-6);\n  color: var(--violet-12);\n}"
            tailwind_code="view! {\n    <Toggle default_pressed=false class=\"bg-white text-gray-700 h-[35px] w-[35px] rounded flex items-center justify-center shadow-md cursor-pointer hover:bg-violet-100 data-[state=on]:bg-violet-200 data-[state=on]:text-violet-900\">\n        <span class=\"font-bold\">\"B\"</span>\n    </Toggle>\n}"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-toggle"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import the component."</p>
        <div class="anatomy-block">{"use leptix_toggle::*;\n\nview! {\n    <Toggle>\n        \"B\"\n    </Toggle>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"The toggle button."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"pressed"</td><td>"MaybeProp<bool>"</td><td>"The controlled pressed state of the toggle."</td></tr>
                    <tr><td>"default_pressed"</td><td>"MaybeProp<bool>"</td><td>"The pressed state when initially rendered. Use when you do not need to control its state."</td></tr>
                    <tr><td>"on_pressed_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the pressed state changes."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the toggle."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"on\" | \"off\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/button" style="color:var(--text-link)">"WAI-ARIA Toggle Button design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Toggles the component."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Toggles the component."</td></tr>
            </tbody>
        </table>
    }
}
