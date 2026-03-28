use leptix_ui::checkbox::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    view! {
        <h1>"Checkbox"</h1>
        <p class="description">
            "A control that allows the user to toggle between checked, unchecked, and indeterminate states."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="checkbox">
            <div class="hero-demo-card">
            <div style="display:flex;align-items:center;gap:8px">
                <Checkbox attr:class="demo-checkbox" default_checked=CheckedState::True>
                    <CheckboxIndicator attr:class="demo-checkbox-indicator">
                        "\u{2713}"
                    </CheckboxIndicator>
                </Checkbox>
                <label class="demo-label">"Accept terms and conditions"</label>
            </div>
            </div>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports indeterminate state."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports form submission with a hidden input."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-checkbox"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_checkbox::*;\n\nview! {\n    <Checkbox>\n        <CheckboxIndicator />\n    </Checkbox>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a checkbox. Renders a button element."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"checked"</td><td>"MaybeProp<CheckedState>"</td><td>"The controlled checked state of the checkbox."</td></tr>
                    <tr><td>"default_checked"</td><td>"MaybeProp<CheckedState>"</td><td>"The checked state when initially rendered. Use when you do not need to control its state."</td></tr>
                    <tr><td>"on_checked_change"</td><td>"Callback<CheckedState>"</td><td>"Event handler called when the checked state changes."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the checkbox."</td></tr>
                    <tr><td>"required"</td><td>"MaybeProp<bool>"</td><td>"When true, indicates that the user must check the checkbox before the form can be submitted."</td></tr>
                    <tr><td>"name"</td><td>"MaybeProp<String>"</td><td>"The name of the checkbox. Submitted with its owning form as part of a name/value pair."</td></tr>
                    <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The value given as data when submitted with a name. Default: \"on\"."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"checked\" | \"unchecked\" | \"indeterminate\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        <h3 id="indicator">"Indicator"</h3>
        <p>"Renders when the checkbox is in a checked or indeterminate state. You can style this element directly, or use it as a wrapper to put an icon into, or both."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"force_mount"</td><td>"MaybeProp<bool>"</td><td>"Used to force mounting when more control is needed. Useful when controlling animation with animation libraries."</td></tr>
                    <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto the child element instead of rendering a default span."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"checked\" | \"unchecked\" | \"indeterminate\""</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/checkbox" style="color:var(--text-link)">"Checkbox WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Toggles the checkbox."</td></tr>
            </tbody>
        </table>
    }
}
