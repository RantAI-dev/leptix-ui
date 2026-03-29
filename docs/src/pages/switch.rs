use leptix_ui::switch::*;
use leptos::prelude::*;

#[component]
pub fn SwitchPage() -> impl IntoView {
    view! {
        <h1>"Switch"</h1>
        <p class="description">
            "A control that allows the user to toggle between checked and not checked."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="switch">
            <div class="hero-demo-card">
            <div style="display:flex;align-items:center;gap:12px">
                <label class="demo-label" r#for="airplane-mode">"Airplane Mode"</label>
                <Switch attr:class="demo-switch" default_checked=true>
                    <SwitchThumb attr:class="demo-switch-thumb" />
                </Switch>
            </div>
            </div>
        </div>
        <div class="code-block">
            <div class="code-block-header">
                <span class="code-block-lang">"styles.css"</span>
            </div>
            <pre>{".SwitchRoot {\n  width: 42px;\n  height: 25px;\n  background-color: var(--black-a9);\n  border-radius: 9999px;\n  position: relative;\n}\n.SwitchRoot[data-state=\"checked\"] {\n  background-color: black;\n}\n\n.SwitchThumb {\n  display: block;\n  width: 21px;\n  height: 21px;\n  background-color: white;\n  border-radius: 9999px;\n  transition: transform 100ms;\n  transform: translateX(2px);\n}\n.SwitchThumb[data-state=\"checked\"] {\n  transform: translateX(19px);\n}"}</pre>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports form submission with a hidden input."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Accessible by default via WAI-ARIA switch role."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-switch"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_switch::*;\n\nview! {\n    <Switch>\n        <SwitchThumb />\n    </Switch>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a switch. Renders a button element."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"checked"</td><td>"MaybeProp<bool>"</td><td>"The controlled checked state of the switch."</td></tr>
                    <tr><td>"default_checked"</td><td>"MaybeProp<bool>"</td><td>"The checked state when initially rendered. Use when you do not need to control its state."</td></tr>
                    <tr><td>"on_checked_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the checked state changes."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the switch."</td></tr>
                    <tr><td>"required"</td><td>"MaybeProp<bool>"</td><td>"When true, indicates that the user must check the switch before the form can be submitted."</td></tr>
                    <tr><td>"name"</td><td>"MaybeProp<String>"</td><td>"The name of the switch. Submitted with its owning form as part of a name/value pair."</td></tr>
                    <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The value given as data when submitted with a name. Default: \"on\"."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"checked\" | \"unchecked\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        <h3 id="thumb">"Thumb"</h3>
        <p>"The thumb that is used to visually indicate whether the switch is on or off."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto the child element instead of rendering a default span."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"checked\" | \"unchecked\""</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/switch" style="color:var(--text-link)">"Switch WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Toggles the switch."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Toggles the switch."</td></tr>
            </tbody>
        </table>
    }
}
