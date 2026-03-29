use leptos::prelude::*;

#[component]
pub fn DirectionProviderPage() -> impl IntoView {
    view! {
        <h1>"Direction Provider"</h1>
        <p class="description">
            "Wraps your application to provide a global reading direction (LTR or RTL) to all Leptix components."
        </p>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the utility from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-direction"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import the provider and wrap your application or a subtree."</p>
        <div class="anatomy-block">{"use leptix_direction::*;\n\nview! {\n    <DirectionProvider dir=\"rtl\">\n        // Your app or component tree\n    </DirectionProvider>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Provides a direction context to all child components."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"dir"</td><td>"String"</td><td>"The reading direction. Must be \"ltr\" or \"rtl\"."</td></tr>
                    <tr><td>"children"</td><td>"Children"</td><td>"The child component tree that will inherit this direction."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Usage ----
        <h2 id="usage">"Usage"</h2>
        <p>"By default, Leptix components inherit the reading direction from the "<code>"dir"</code>" attribute on the "<code>"<html>"</code>" element. "<code>"DirectionProvider"</code>" is useful when you need to override this for a subtree, or when your app does not set "<code>"dir"</code>" on the root element."</p>

        <h3>"Global RTL Support"</h3>
        <div class="anatomy-block">{"use leptix_direction::*;\n\nview! {\n    <DirectionProvider dir=\"rtl\">\n        <App />\n    </DirectionProvider>\n}"}</div>

        <h3 style="margin-top:1.5rem">"Per-Section Direction"</h3>
        <p>"You can nest providers to override direction for specific sections of your UI."</p>
        <div class="anatomy-block">{"view! {\n    <DirectionProvider dir=\"ltr\">\n        // This section is left-to-right\n        <MainContent />\n\n        <DirectionProvider dir=\"rtl\">\n            // This section is right-to-left\n            <ArabicContent />\n        </DirectionProvider>\n    </DirectionProvider>\n}"}</div>
    }
}
