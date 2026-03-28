use leptix_ui::checkbox::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    view! {
        <h1>"Checkbox"</h1>
        <p class="subtitle">"A control that allows the user to toggle between checked, unchecked, and indeterminate states."</p>

        <div class="features">
            <span class="feature">"Tri-state"</span>
            <span class="feature">"Keyboard toggle"</span>
            <span class="feature">"Accessible"</span>
            <span class="feature">"Form ready"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <div style="display:flex;align-items:center;gap:8px">
                    <Checkbox attr:class="demo-checkbox" default_checked=CheckedState::True>
                        <CheckboxIndicator attr:class="demo-checkbox-indicator">
                            "\u{2713}"
                        </CheckboxIndicator>
                    </Checkbox>
                    <label class="demo-label">"Accept terms and conditions"</label>
                </div>
            </div>
            <div class="demo-code">{"<Checkbox default_checked=CheckedState::True>\n  <CheckboxIndicator>\n    \"\u{2713}\"\n  </CheckboxIndicator>\n</Checkbox>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Checkbox"</h3>
        <p>"Contains all the parts of a checkbox. Renders a button element."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"checked"</td><td>"MaybeProp<CheckedState>"</td><td>"Controlled checked state."</td></tr>
                <tr><td>"default_checked"</td><td>"MaybeProp<CheckedState>"</td><td>"Uncontrolled initial checked state."</td></tr>
                <tr><td>"on_checked_change"</td><td>"Callback<CheckedState>"</td><td>"Called when the checked state changes."</td></tr>
                <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents user interaction."</td></tr>
                <tr><td>"required"</td><td>"MaybeProp<bool>"</td><td>"When true, indicates the field is required."</td></tr>
                <tr><td>"name"</td><td>"MaybeProp<String>"</td><td>"The name of the checkbox for form submission."</td></tr>
                <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The value for form submission. Default: \"on\"."</td></tr>
            </tbody>
        </table>

        <h3>"CheckboxIndicator"</h3>
        <p>"Renders when the checkbox is in a checked or indeterminate state. Hidden otherwise."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"force_mount"</td><td>"MaybeProp<bool>"</td><td>"When true, the indicator is always rendered."</td></tr>
                <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto the child element instead of rendering a default span."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Toggles the checkbox."</td></tr>
            </tbody>
        </table>
    }
}
