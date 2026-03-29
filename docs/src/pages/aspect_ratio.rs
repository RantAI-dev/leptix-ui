use crate::HeroCodeBlock;
use leptix_ui::aspect_ratio::*;
use leptos::prelude::*;

#[component]
pub fn AspectRatioPage() -> impl IntoView {
    view! {
        <h1>"Aspect Ratio"</h1>
        <p class="description">
            "Displays content within a desired ratio."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="aspect-ratio">
            <div class="hero-demo-card">
            <div class="demo-aspect-ratio">
                <AspectRatio ratio=Signal::derive(|| 16.0 / 9.0)>
                    <div style="width:100%;height:100%;background:linear-gradient(135deg,#5cb6f9,#050a30);border-radius:8px;display:flex;align-items:center;justify-content:center;color:#fff;font-weight:600;font-size:1.1rem;">
                        "16 : 9"
                    </div>
                </AspectRatio>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            usage_code="use leptix_ui::aspect_ratio::*;\n\nview! {\n    <AspectRatio ratio=Signal::derive(|| 16.0 / 9.0)>\n        <img src=\"landscape.jpg\" style=\"object-fit: cover; width: 100%; height: 100%;\" />\n    </AspectRatio>\n}"
            css_code=".AspectRatioRoot {\n  width: 300px;\n}\n\n.AspectRatioRoot img {\n  object-fit: cover;\n  width: 100%;\n  height: 100%;\n  border-radius: 6px;\n}"
            css_modules_code=".Root {\n  width: 300px;\n}\n\n.Root img {\n  object-fit: cover;\n  width: 100%;\n  height: 100%;\n  border-radius: 6px;\n}"
            tailwind_code="view! {\n    <AspectRatio ratio=Signal::derive(|| 16.0 / 9.0) class=\"w-[300px]\">\n        <img src=\"landscape.jpg\" class=\"object-cover w-full h-full rounded-md\" />\n    </AspectRatio>\n}"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Accepts any custom ratio."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Renders a single wrapper element."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-aspect-ratio"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_aspect_ratio::*;\n\nview! {\n    <AspectRatio ratio=Signal::derive(|| 16.0 / 9.0)>\n        // content\n    </AspectRatio>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains the content you want to constrain to a given ratio."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"ratio"</td><td>"Signal<f64>"</td><td>"The desired ratio. E.g. 16.0 / 9.0."</td></tr>
                </tbody>
            </table>
        </div>
    }
}
