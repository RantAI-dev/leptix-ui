use leptix_ui::toggle_group::*;
use leptos::prelude::*;

#[component]
pub fn ToggleGroupPage() -> impl IntoView {
    view! {
        <h1>"Toggle Group"</h1>
        <p class="description">
            "A set of two-state buttons that can be toggled on or off."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="toggle-group">
            <div class="hero-demo-card">
            <div class="demo-toggle-group">
                <ToggleGroup r#type=ToggleGroupType::Single default_value=vec!["bold".to_string()]>
                    <ToggleGroupItem value="bold">"B"</ToggleGroupItem>
                    <ToggleGroupItem value="italic">"I"</ToggleGroupItem>
                    <ToggleGroupItem value="underline">"U"</ToggleGroupItem>
                </ToggleGroup>
            </div>
            </div>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports horizontal/vertical orientation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports single and multiple pressed buttons."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-toggle-group"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_toggle_group::*;\n\nview! {\n    <ToggleGroup r#type=ToggleGroupType::Single>\n        <ToggleGroupItem value=\"bold\">\"B\"</ToggleGroupItem>\n        <ToggleGroupItem value=\"italic\">\"I\"</ToggleGroupItem>\n    </ToggleGroup>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a toggle group."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"type"</td><td>"ToggleGroupType"</td><td>"Determines whether a single or multiple items can be pressed at the same time."</td></tr>
                    <tr><td>"value"</td><td>"MaybeProp<Vec<String>>"</td><td>"The controlled value of the pressed item(s)."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<Vec<String>>"</td><td>"The value of the item(s) to be pressed when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<Vec<String>>"</td><td>"Event handler called when the pressed state of an item changes."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the toggle group and all its items."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="item">"Item"</h3>
        <p>"An item in the group."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"String"</td><td>"A unique value for the item."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the item."</td></tr>
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
        <p>"Uses "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/toolbar" style="color:var(--text-link)">"roving tabindex"</a>" to manage focus movement among items."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus to either the pressed item or the first item in the group."</td></tr>
                <tr><td><kbd>"Space"</kbd></td><td>"Activates/deactivates the item."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Activates/deactivates the item."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Moves focus to the next item in the group."</td></tr>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Moves focus to the next item in the group."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Moves focus to the previous item in the group."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Moves focus to the previous item in the group."</td></tr>
                <tr><td><kbd>"Home"</kbd></td><td>"Moves focus to the first item."</td></tr>
                <tr><td><kbd>"End"</kbd></td><td>"Moves focus to the last item."</td></tr>
            </tbody>
        </table>
    }
}
