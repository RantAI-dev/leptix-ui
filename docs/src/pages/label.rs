use crate::HeroCodeBlock;
use leptix_ui::label::*;
use leptos::prelude::*;

#[component]
pub fn LabelPage() -> impl IntoView {
    view! {
        <h1>"Label"</h1>
        <p class="description">
            "Renders an accessible label associated with controls."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="label">
            <div class="hero-demo-card">
            <div style="display:flex;flex-direction:column;gap:8px">
                <Label attr:class="demo-label" attr:r#for="demo-email">
                    "Email address"
                </Label>
                <input id="demo-email" class="demo-input" type="email" placeholder="you@example.com" />
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_label::*;\n\nview! {\n    <Label attr:class=\"Label\" r#for=\"email\">\"Email address\"</Label>\n    <input id=\"email\" type=\"email\" />\n}"
            css_styles=".Label {\n  color: white;\n  font-size: 15px;\n  line-height: 35px;\n  font-weight: 500;\n  user-select: none;\n}\n\n.Label[data-disabled] {\n  color: var(--mauve-8);\n  cursor: not-allowed;\n}"
            modules_usage="// import styles from \"./label.module.css\";\nuse leptix_label::*;\n\nview! {\n    <Label attr:class=styles.root r#for=\"email\">\"Email address\"</Label>\n    <input id=\"email\" type=\"email\" />\n}"
            modules_styles=".root {\n  color: white;\n  font-size: 15px;\n  line-height: 35px;\n  font-weight: 500;\n  user-select: none;\n}\n\n.root[data-disabled] {\n  color: var(--mauve-8);\n  cursor: not-allowed;\n}"
            tailwind_usage="use leptix_label::*;\n\nview! {\n    <Label attr:class=\"text-white text-[15px] leading-[35px] font-medium select-none data-[disabled]:text-gray-500 data-[disabled]:cursor-not-allowed\" r#for=\"email\">\n        \"Email address\"\n    </Label>\n    <input id=\"email\" type=\"email\" />\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      colors: {\n        label: {\n          DEFAULT: \"#ffffff\",\n          disabled: \"var(--mauve-8)\",\n        },\n      },\n      fontSize: {\n        label: \"15px\",\n      },\n      lineHeight: {\n        label: \"35px\",\n      },\n    },\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Text selection is prevented when double clicking label."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports nested controls."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-label"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import the component."</p>
        <div class="anatomy-block">{"use leptix_label::*;\n\nview! {\n    <Label r#for=\"email\">\"Email address\"</Label>\n    <input id=\"email\" type=\"email\" />\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains the content for the label."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"r#for"</td><td>"String"</td><td>"The id of the element the label is associated with."</td></tr>
                    <tr><td>"on_mouse_down"</td><td>"MaybeCallback<MouseEvent>"</td><td>"Event handler called on mouse down. Prevents text selection on double-click by default."</td></tr>
                    <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto the child element instead of rendering a default label."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Uses a native "<code>"label"</code>" element under the hood."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"N/A"</kbd></td><td>"Label is a native element. Clicking the label focuses its associated control via the for attribute."</td></tr>
            </tbody>
        </table>
    }
}
