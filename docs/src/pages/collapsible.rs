use crate::HeroCodeBlock;
use leptix_ui::collapsible::*;
use leptos::prelude::*;

#[component]
pub fn CollapsiblePage() -> impl IntoView {
    view! {
        <h1>"Collapsible"</h1>
        <p class="description">
            "An interactive component which expands and collapses a panel."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="collapsible">
            <div class="hero-demo-card">
            <Collapsible attr:class="demo-collapsible">
                <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:8px">
                    <span style="font-size:14px;font-weight:500">"Tagged items"</span>
                    <CollapsibleTrigger attr:class="demo-btn" attr:style="padding:4px 10px;font-size:12px">
                        "Toggle"
                    </CollapsibleTrigger>
                </div>

                <div style="padding:8px 12px;border:1px solid var(--border);border-radius:6px;font-size:14px;margin-bottom:4px">
                    "@leptix/primitives"
                </div>

                <CollapsibleContent attr:class="demo-collapsible-content">
                    <div style="padding:8px 12px;border:1px solid var(--border);border-radius:6px;font-size:14px;margin-bottom:4px">
                        "@leptix/colors"
                    </div>
                    <div style="padding:8px 12px;border:1px solid var(--border);border-radius:6px;font-size:14px;margin-bottom:4px">
                        "@leptix/icons"
                    </div>
                </CollapsibleContent>
            </Collapsible>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_collapsible::*;\n\nview! {\n    <Collapsible attr:class=\"CollapsibleRoot\">\n        <CollapsibleTrigger>\"Toggle\"</CollapsibleTrigger>\n        <CollapsibleContent attr:class=\"CollapsibleContent\">\n            \"Hidden content here.\"\n        </CollapsibleContent>\n    </Collapsible>\n}"
            css_styles=".CollapsibleRoot {\n  width: 300px;\n}\n\n.CollapsibleContent {\n  overflow: hidden;\n}\n.CollapsibleContent[data-state=\"open\"] {\n  animation: slideDown 300ms ease-out;\n}\n.CollapsibleContent[data-state=\"closed\"] {\n  animation: slideUp 300ms ease-out;\n}\n\n@keyframes slideDown {\n  from { height: 0; }\n  to { height: var(--leptix-collapsible-content-height); }\n}\n@keyframes slideUp {\n  from { height: var(--leptix-collapsible-content-height); }\n  to { height: 0; }\n}"
            modules_usage="// import styles from \"./collapsible.module.css\";\nuse leptix_collapsible::*;\n\nview! {\n    <Collapsible attr:class=styles.root>\n        <CollapsibleTrigger>\"Toggle\"</CollapsibleTrigger>\n        <CollapsibleContent attr:class=styles.content>\n            \"Hidden content here.\"\n        </CollapsibleContent>\n    </Collapsible>\n}"
            modules_styles=".root {\n  width: 300px;\n}\n\n.content {\n  overflow: hidden;\n}\n.content[data-state=\"open\"] {\n  animation: slideDown 300ms ease-out;\n}\n.content[data-state=\"closed\"] {\n  animation: slideUp 300ms ease-out;\n}\n\n@keyframes slideDown {\n  from { height: 0; }\n  to { height: var(--leptix-collapsible-content-height); }\n}\n@keyframes slideUp {\n  from { height: var(--leptix-collapsible-content-height); }\n  to { height: 0; }\n}"
            tailwind_usage="use leptix_collapsible::*;\n\nview! {\n    <Collapsible attr:class=\"w-[300px]\">\n        <CollapsibleTrigger>\"Toggle\"</CollapsibleTrigger>\n        <CollapsibleContent attr:class=\"overflow-hidden data-[state=open]:animate-slideDown data-[state=closed]:animate-slideUp\">\n            \"Hidden content here.\"\n        </CollapsibleContent>\n    </Collapsible>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      keyframes: {\n        slideDown: {\n          from: { height: \"0\" },\n          to: { height: \"var(--leptix-collapsible-content-height)\" },\n        },\n        slideUp: {\n          from: { height: \"var(--leptix-collapsible-content-height)\" },\n          to: { height: \"0\" },\n        },\n      },\n      animation: {\n        slideDown: \"slideDown 300ms ease-out\",\n        slideUp: \"slideUp 300ms ease-out\",\n      },\n    },\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-collapsible"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_collapsible::*;\n\nview! {\n    <Collapsible>\n        <CollapsibleTrigger />\n        <CollapsibleContent />\n    </Collapsible>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a collapsible."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state of the collapsible."</td></tr>
                    <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"The open state when initially rendered. Use when you do not need to control its open state."</td></tr>
                    <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the open state changes."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the collapsible."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that toggles the collapsible."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto the child element instead of rendering a default button."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        <h3 id="content">"Content"</h3>
        <p>"The component that contains the collapsible content."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"force_mount"</td><td>"MaybeProp<bool>"</td><td>"Used to force mounting when more control is needed. Useful when controlling animation with animation libraries."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/disclosure" style="color:var(--text-link)">"Disclosure WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Opens/closes the collapsible."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Opens/closes the collapsible."</td></tr>
            </tbody>
        </table>
    }
}
