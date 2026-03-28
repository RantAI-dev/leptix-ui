use leptix_ui::toggle::*;
use leptos::prelude::*;

#[component]
pub fn TogglePage() -> impl IntoView {
    view! {
        <h1>"Toggle"</h1>
        <p class="subtitle">"A two-state button that can be either on or off."</p>

        <div class="features">
            <span class="feature">"Keyboard toggle"</span>
            <span class="feature">"Accessible"</span>
            <span class="feature">"Controlled / uncontrolled"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <Toggle attr:class="demo-toggle" default_pressed=false>
                    <span style="font-size:16px;font-weight:600">"B"</span>
                </Toggle>
            </div>
            <div class="demo-code">{"<Toggle default_pressed=false>\n  \"B\"\n</Toggle>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Toggle"</h3>
        <p>"A button that toggles between on and off states. Renders a button element."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"pressed"</td><td>"MaybeProp<bool>"</td><td>"Controlled pressed state."</td></tr>
                <tr><td>"default_pressed"</td><td>"MaybeProp<bool>"</td><td>"Uncontrolled initial pressed state. Default: false."</td></tr>
                <tr><td>"on_pressed_change"</td><td>"Callback<bool>"</td><td>"Called when the pressed state changes."</td></tr>
                <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents user interaction."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Toggles the pressed state."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Toggles the pressed state."</td></tr>
            </tbody>
        </table>
    }
}
