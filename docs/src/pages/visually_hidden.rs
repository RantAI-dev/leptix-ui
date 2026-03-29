use leptos::prelude::*;

#[component]
pub fn VisuallyHiddenPage() -> impl IntoView {
    view! {
        <h1>"Visually Hidden"</h1>
        <p class="description">
            "Hides content from the screen while keeping it accessible to screen readers."
        </p>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the utility from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-visually-hidden"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import the component and wrap the text you want to hide visually."</p>
        <div class="anatomy-block">{"use leptix_visually_hidden::*;\n\nview! {\n    <VisuallyHidden>\n        \"Screen reader only text\"\n    </VisuallyHidden>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Renders a "<code>"<span>"</code>" element that is visually hidden but remains in the accessibility tree."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"children"</td><td>"Children"</td><td>"The content to hide visually."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Usage ----
        <h2 id="usage">"Usage"</h2>
        <p>"Use "<code>"VisuallyHidden"</code>" whenever you have visual cues that need textual equivalents for screen readers."</p>

        <h3>"Icon Buttons"</h3>
        <p>"When a button only contains an icon, add a visually hidden label:"</p>
        <div class="anatomy-block">{"view! {\n    <button>\n        <VisuallyHidden>\"Close\"</VisuallyHidden>\n        <CrossIcon />\n    </button>\n}"}</div>

        <h3 style="margin-top:1.5rem">"Supplementary Descriptions"</h3>
        <p>"Add extra context for assistive technology without affecting visual layout:"</p>
        <div class="anatomy-block">{"view! {\n    <h2>\n        \"Annual Report\"\n        <VisuallyHidden>\" - Fiscal Year 2025\"</VisuallyHidden>\n    </h2>\n}"}</div>

        <h3 style="margin-top:1.5rem">"How It Works"</h3>
        <p>"The component applies CSS that positions the element off-screen with a 1px size, ensuring it is not visible but still read by assistive technology. This is preferred over "<code>"display: none"</code>" or "<code>"visibility: hidden"</code>", which remove elements from the accessibility tree entirely."</p>
    }
}
