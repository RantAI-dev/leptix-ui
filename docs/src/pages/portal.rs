use leptos::prelude::*;

#[component]
pub fn PortalPage() -> impl IntoView {
    view! {
        <h1>"Portal"</h1>
        <p class="description">
            "Renders a component tree into a different part of the DOM, typically at the end of "<code>"document.body"</code>"."
        </p>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the utility from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-portal"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import the component and wrap the content you want to portal."</p>
        <div class="anatomy-block">{"use leptix_portal::*;\n\nview! {\n    <Portal>\n        // Content rendered at the end of document.body\n        <div>\"Portaled content\"</div>\n    </Portal>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Appends its children to a specified DOM element."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"container"</td><td>"MaybeProp<web_sys::Element>"</td><td>"An optional DOM element to portal into. Defaults to document.body."</td></tr>
                    <tr><td>"children"</td><td>"Children"</td><td>"The content to render inside the portal."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Usage ----
        <h2 id="usage">"Usage"</h2>
        <p>"Portal is used internally by components like "<code>"Dialog"</code>", "<code>"AlertDialog"</code>", "<code>"Menubar"</code>", and any component that renders overlays. It ensures content escapes parent clipping and stacking contexts."</p>

        <h3>"Custom Container"</h3>
        <p>"You can portal content into a specific element rather than "<code>"document.body"</code>"."</p>
        <div class="anatomy-block">{"use leptix_portal::*;\n\nlet container = create_node_ref::<html::Div>();\n\nview! {\n    <div node_ref=container />\n\n    <Portal container=container>\n        <div>\"Renders inside the div above\"</div>\n    </Portal>\n}"}</div>

        <h3 style="margin-top:1.5rem">"Why Use a Portal?"</h3>
        <p>"Portals solve common CSS issues with overlays. When a dropdown or dialog is rendered inside a parent with "<code>"overflow: hidden"</code>" or a restrictive "<code>"z-index"</code>", the overlay may be clipped or hidden. Portaling the content to "<code>"document.body"</code>" avoids these issues entirely."</p>
    }
}
