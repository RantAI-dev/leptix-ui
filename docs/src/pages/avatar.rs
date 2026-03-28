use leptix_ui::avatar::*;
use leptos::prelude::*;

#[component]
pub fn AvatarPage() -> impl IntoView {
    view! {
        <h1>"Avatar"</h1>
        <p class="description">
            "An image element with a fallback for representing the user."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="avatar">
            <div class="hero-demo-card">
            <div style="display:flex;align-items:center;gap:16px">
                <Avatar attr:class="demo-avatar">
                    <AvatarImage src="https://i.pravatar.cc/48?u=leptix" />
                    <AvatarFallback attr:class="demo-avatar-fallback">"JD"</AvatarFallback>
                </Avatar>

                <Avatar attr:class="demo-avatar">
                    <AvatarImage src="https://broken.invalid/avatar.png" />
                    <AvatarFallback attr:class="demo-avatar-fallback">"PD"</AvatarFallback>
                </Avatar>
            </div>
            </div>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Automatic and manual control over when the image renders."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Fallback part accepts any children."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Optionally delay fallback rendering to avoid content flashing."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-avatar"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_avatar::*;\n\nview! {\n    <Avatar>\n        <AvatarImage />\n        <AvatarFallback />\n    </Avatar>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of an avatar."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto the child element instead of rendering a default span."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="image">"Image"</h3>
        <p>"The image to render. By default it will only render when it has loaded."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"src"</td><td>"MaybeProp<String>"</td><td>"The image source URL."</td></tr>
                    <tr><td>"on_loading_status_change"</td><td>"Callback<ImageLoadingStatus>"</td><td>"A callback providing information about the loading status of the image."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="fallback">"Fallback"</h3>
        <p>"An element that renders when the image hasn\u{2019}t loaded. This means whilst it\u{2019}s loading, or if there was an error. If you notice a flash during loading, you can provide a delay_ms prop to delay its rendering."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"delay_ms"</td><td>"MaybeProp<i32>"</td><td>"Useful for delaying rendering so it only appears for those with slower connections."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Follows the "<a href="https://www.w3.org/WAI/ARIA/apg" style="color:var(--text-link)">"WAI-ARIA guidelines"</a>" for images."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"N/A"</kbd></td><td>"Avatar is a presentational component and does not require keyboard interaction."</td></tr>
            </tbody>
        </table>
    }
}
