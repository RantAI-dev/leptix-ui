use leptix_ui::select::*;
use leptos::prelude::*;

#[component]
pub fn SelectPage() -> impl IntoView {
    view! {
        <h1>"Select"</h1>
        <p class="description">
            "Displays a list of options for the user to pick from, triggered by a button."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="select">
            <div class="hero-demo-card">
            <div class="demo-select">
                <Select>
                    <SelectTrigger>
                        <SelectValue placeholder="Select a fruit" />
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent>
                            <SelectViewport>
                                <SelectItem value="apple">
                                    <SelectItemText>"Apple"</SelectItemText>
                                </SelectItem>
                                <SelectItem value="banana">
                                    <SelectItemText>"Banana"</SelectItemText>
                                </SelectItem>
                                <SelectItem value="cherry">
                                    <SelectItemText>"Cherry"</SelectItemText>
                                </SelectItem>
                            </SelectViewport>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </div>
            </div>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports items, labels, groups of items."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Focus is fully managed."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Typeahead support."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-select"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_select::*;\n\nview! {\n    <Select>\n        <SelectTrigger>\n            <SelectValue placeholder=\"Pick one\" />\n        </SelectTrigger>\n        <SelectPortal>\n            <SelectContent>\n                <SelectViewport>\n                    <SelectItem value=\"…\">\n                        <SelectItemText />\n                    </SelectItem>\n                </SelectViewport>\n            </SelectContent>\n        </SelectPortal>\n    </Select>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a select."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The controlled value."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<String>"</td><td>"The value when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<String>"</td><td>"Event handler called when the value changes."</td></tr>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents interaction."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that toggles the select. The SelectContent will position itself relative to the trigger."</p>

        <h3 id="value">"Value"</h3>
        <p>"The part that reflects the selected value."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"placeholder"</td><td>"&str"</td><td>"Text shown when no value is selected."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="item">"Item"</h3>
        <p>"A selectable item in the list."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"String"</td><td>"The value given as data when submitted with a name."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the item."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/listbox" style="color:var(--text-link)">"ListBox WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"When focus is on trigger, opens the select and focuses the selected or first item. When focus is on an item, selects it."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"When focus is on trigger, opens the select. When focus is on an item, selects it."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"When focus is on trigger, opens the select. When open, moves focus to the next item."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"When focus is on trigger, opens the select. When open, moves focus to the previous item."</td></tr>
                <tr><td><kbd>"Escape"</kbd></td><td>"Closes the select and moves focus back to the trigger."</td></tr>
            </tbody>
        </table>
    }
}
