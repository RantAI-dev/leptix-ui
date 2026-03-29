use leptos::prelude::*;

#[component]
pub fn AccessibleIconPage() -> impl IntoView {
    view! {
        <h1>"Accessible Icon"</h1>
        <p class="description">
            "Makes icons accessible by adding a label that is announced by screen readers but hidden from the visual interface."
        </p>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-accessible-icon"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import the component and wrap your icon with it."</p>
        <div class="anatomy-block">{"use leptix_accessible_icon::*;\n\nview! {\n    <AccessibleIcon label=\"Close\">\n        // Your icon component or SVG goes here\n        <svg width=\"15\" height=\"15\" viewBox=\"0 0 15 15\">\n            // ...\n        </svg>\n    </AccessibleIcon>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Wraps the icon and provides an accessible label via "<code>"VisuallyHidden"</code>"."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"label"</td><td>"String"</td><td>"The accessible label for the icon. This label will be visually hidden but announced to screen readers."</td></tr>
                    <tr><td>"children"</td><td>"Children"</td><td>"The icon element to render."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Usage ----
        <h2 id="usage">"Usage"</h2>
        <p>"Most icons or icon systems come with no accessibility built-in. For example, when using a "<code>"HamburgerMenuIcon"</code>" for a navigation menu trigger:"</p>
        <div class="anatomy-block">{"// Accessible:\nview! {\n    <AccessibleIcon label=\"Navigation menu\">\n        <HamburgerMenuIcon />\n    </AccessibleIcon>\n}\n\n// Also accessible (using aria-label directly):\nview! {\n    <button aria-label=\"Navigation menu\">\n        <HamburgerMenuIcon />\n    </button>\n}"}</div>

        <p style="margin-top:1.5rem">"When using inside a button, the icon label should describe the action rather than the icon itself. The button already communicates its role, so the label should clarify what it does:"</p>
        <div class="anatomy-block">{"view! {\n    <button>\n        <AccessibleIcon label=\"Close dialog\">\n            <CrossIcon />\n        </AccessibleIcon>\n    </button>\n}"}</div>
    }
}
