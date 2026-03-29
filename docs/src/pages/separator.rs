use leptix_ui::separator::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorPage() -> impl IntoView {
    view! {
        <h1>"Separator"</h1>
        <p class="description">
            "Visually or semantically separates content."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="separator">
            <div class="hero-demo-card">
            <div style="width:300px">
                <div style="font-size:15px;font-weight:600">"Leptix Primitives"</div>
                <div style="font-size:14px;color:var(--text-secondary);margin-top:4px">"An accessible component library for Leptos."</div>
                <Separator attr:class="demo-separator" attr:style="margin:16px 0" />
                <div style="display:flex;align-items:center;gap:12px;font-size:14px">
                    <span>"Blog"</span>
                    <Separator orientation=Orientation::Vertical attr:class="demo-separator" />
                    <span>"Docs"</span>
                    <Separator orientation=Orientation::Vertical attr:class="demo-separator" />
                    <span>"Source"</span>
                </div>
            </div>
            </div>
        </div>
        <div class="code-block">
            <div class="code-block-header">
                <span class="code-block-lang">"styles.css"</span>
            </div>
            <pre>{".SeparatorRoot {\n  background-color: var(--violet-6);\n}\n.SeparatorRoot[data-orientation=\"horizontal\"] {\n  height: 1px;\n  width: 100%;\n}\n.SeparatorRoot[data-orientation=\"vertical\"] {\n  height: 100%;\n  width: 1px;\n  min-height: 20px;\n}"}</pre>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports horizontal and vertical orientations."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports decorative or semantic separation."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-separator"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import the component."</p>
        <div class="anatomy-block">{"use leptix_separator::*;\n\nview! {\n    <Separator />\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"The separator."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"orientation"</td><td>"MaybeProp<Orientation>"</td><td>"The orientation of the separator. Default: Horizontal."</td></tr>
                    <tr><td>"decorative"</td><td>"MaybeProp<bool>"</td><td>"When true, signifies that it is purely visual. The separator will have role=\"none\" and will not be accessible to screen readers. Default: false."</td></tr>
                    <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto the child element instead of rendering a default div."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/landmark" style="color:var(--text-link)">"separator role requirements"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"N/A"</kbd></td><td>"Separator is not interactive. It uses role=\"separator\" with aria-orientation for screen readers, or role=\"none\" when decorative."</td></tr>
            </tbody>
        </table>
    }
}
