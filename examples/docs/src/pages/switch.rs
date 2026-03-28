use leptix_ui::switch::*;
use leptos::prelude::*;

#[component]
pub fn SwitchPage() -> impl IntoView {
    view! {
        <h1>"Switch"</h1>
        <p class="subtitle">"A control that allows the user to toggle between checked and not checked."</p>

        <div class="features">
            <span class="feature">"Keyboard toggle"</span>
            <span class="feature">"Accessible"</span>
            <span class="feature">"Controlled / uncontrolled"</span>
            <span class="feature">"Form ready"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <div style="display:flex;align-items:center;gap:12px">
                    <label class="demo-label" r#for="airplane-mode">"Airplane Mode"</label>
                    <Switch attr:class="demo-switch" default_checked=true>
                        <SwitchThumb attr:class="demo-switch-thumb" />
                    </Switch>
                </div>
            </div>
            <div class="demo-code">{"<Switch default_checked=true>\n  <SwitchThumb />\n</Switch>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Switch"</h3>
        <p>"Contains all the parts of a switch. Renders a button element."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"checked"</td><td>"MaybeProp<bool>"</td><td>"Controlled checked state."</td></tr>
                <tr><td>"default_checked"</td><td>"MaybeProp<bool>"</td><td>"Uncontrolled initial checked state."</td></tr>
                <tr><td>"on_checked_change"</td><td>"Callback<bool>"</td><td>"Called when the checked state changes."</td></tr>
                <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents user interaction."</td></tr>
                <tr><td>"required"</td><td>"MaybeProp<bool>"</td><td>"When true, indicates the field is required."</td></tr>
                <tr><td>"name"</td><td>"MaybeProp<String>"</td><td>"The name of the switch for form submission."</td></tr>
                <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The value for form submission. Default: \"on\"."</td></tr>
            </tbody>
        </table>

        <h3>"SwitchThumb"</h3>
        <p>"The thumb that visually indicates the current state."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto the child element instead of rendering a default span."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Toggles the switch."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Toggles the switch."</td></tr>
            </tbody>
        </table>
    }
}
