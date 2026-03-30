use crate::HeroCodeBlock;
use leptix_ui::hover_card::*;
use leptos::prelude::*;

#[component]
pub fn HoverCardPage() -> impl IntoView {
    view! {
        <h1>"Hover Card"</h1>
        <p class="description">
            "For sighted users to preview content available behind a link."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="hover-card">
            <div class="hero-demo-card">
            <div class="demo-hover-card">
                <HoverCard>
                    <HoverCardTrigger attr:style="text-decoration:none;display:inline-flex;align-items:center;gap:8px;padding:8px 16px;border-radius:999px;cursor:pointer;color:var(--accent);font-weight:600;font-size:14px;background:var(--bg-panel);border:1px solid var(--border);transition:box-shadow 0.15s">
                        <img src="https://avatars.githubusercontent.com/u/208423893?s=24" width="24" height="24" style="border-radius:50%" alt="" />
                        "@RantAI-dev"
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent attr:style="background:var(--bg-panel);border-radius:8px;padding:20px;box-shadow:0 10px 38px -10px rgba(22,23,24,0.35),0 10px 20px -15px rgba(22,23,24,0.2);width:300px;z-index:9999">
                            <div style="display:flex;gap:16px;align-items:flex-start">
                                <img src="https://avatars.githubusercontent.com/u/208423893?s=48" width="48" height="48" style="border-radius:50%;flex-shrink:0" alt="" />
                                <div>
                                    <div style="font-weight:600;font-size:15px;margin-bottom:2px">"RantAI"</div>
                                    <div style="font-size:13px;color:#888;margin-bottom:10px">"@RantAI-dev"</div>
                                    <p style="font-size:14px;color:#888;line-height:1.5;margin:0">"Building intelligent developer tools and open-source infrastructure for the Rust ecosystem."</p>
                                </div>
                            </div>
                            <HoverCardArrow />
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCard>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_ui::hover_card::*;\n\nview! {\n    <HoverCard>\n        <HoverCardTrigger attr:class=\"HoverCardTrigger\">\"Hover over @leptix\"</HoverCardTrigger>\n        <HoverCardPortal>\n            <HoverCardContent attr:class=\"HoverCardContent\">\n                <p>\"Preview content goes here.\"</p>\n                <HoverCardArrow attr:class=\"HoverCardArrow\" />\n            </HoverCardContent>\n        </HoverCardPortal>\n    </HoverCard>\n}"
            css_styles=".HoverCardContent {\n  border-radius: 6px;\n  padding: 20px;\n  width: 300px;\n  background-color: white;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.HoverCardTrigger {\n  cursor: pointer;\n  color: var(--violet-11);\n  font-weight: 500;\n}\n\n.HoverCardArrow {\n  fill: white;\n}"
            modules_usage="use leptix_ui::hover_card::*;\n// use styles from module\n\nview! {\n    <HoverCard>\n        <HoverCardTrigger attr:class=styles.trigger>\"Hover over @leptix\"</HoverCardTrigger>\n        <HoverCardPortal>\n            <HoverCardContent attr:class=styles.content>\n                <p>\"Preview content goes here.\"</p>\n                <HoverCardArrow attr:class=styles.arrow />\n            </HoverCardContent>\n        </HoverCardPortal>\n    </HoverCard>\n}"
            modules_styles=".content {\n  border-radius: 6px;\n  padding: 20px;\n  width: 300px;\n  background-color: white;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.trigger {\n  cursor: pointer;\n  color: var(--violet-11);\n  font-weight: 500;\n}\n\n.arrow {\n  fill: white;\n}"
            tailwind_usage="use leptix_ui::hover_card::*;\n\nview! {\n    <HoverCard>\n        <HoverCardTrigger attr:class=\"cursor-pointer text-violet-700 font-medium\">\n            \"Hover over @leptix\"\n        </HoverCardTrigger>\n        <HoverCardPortal>\n            <HoverCardContent attr:class=\"rounded-md p-5 w-[300px] bg-white shadow-lg\">\n                <p>\"Preview content goes here.\"</p>\n                <HoverCardArrow attr:class=\"fill-white\" />\n            </HoverCardContent>\n        </HoverCardPortal>\n    </HoverCard>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      width: {\n        hovercard: \"300px\",\n      },\n    },\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Customize side, alignment, offsets."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Optionally render a pointing arrow."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports configurable open and close delays."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-hover-card"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_hover_card::*;\n\nview! {\n    <HoverCard>\n        <HoverCardTrigger />\n        <HoverCardPortal>\n            <HoverCardContent>\n                <HoverCardArrow />\n            </HoverCardContent>\n        </HoverCardPortal>\n    </HoverCard>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a hover card."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state of the hover card."</td></tr>
                    <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"The open state of the hover card when it is initially rendered."</td></tr>
                    <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the open state of the hover card changes."</td></tr>
                    <tr><td>"open_delay"</td><td>"MaybeProp<i32>"</td><td>"The duration from when the mouse enters the trigger until the hover card opens. Default: 700ms."</td></tr>
                    <tr><td>"close_delay"</td><td>"MaybeProp<i32>"</td><td>"The duration from when the mouse leaves the trigger or content until the hover card closes. Default: 300ms."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The link that opens the hover card when hovered."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
            </tbody>
        </table>

        <h3 id="portal">"Portal"</h3>
        <p>"When used, portals the content part into the body."</p>

        <h3 id="content">"Content"</h3>
        <p>"The component that pops out when the hover card is open."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"side"</td><td>"MaybeProp<String>"</td><td>"The preferred side of the trigger to render against. Default: \"bottom\"."</td></tr>
                    <tr><td>"side_offset"</td><td>"MaybeProp<f64>"</td><td>"The distance in pixels from the trigger."</td></tr>
                    <tr><td>"align"</td><td>"MaybeProp<String>"</td><td>"The preferred alignment against the trigger. Default: \"center\"."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-side]"</td><td>"\"top\" | \"right\" | \"bottom\" | \"left\""</td></tr>
                <tr><td>"[data-align]"</td><td>"\"start\" | \"center\" | \"end\""</td></tr>
            </tbody>
        </table>

        <h3 id="arrow">"Arrow"</h3>
        <p>"An optional arrow element to render alongside the hover card."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"The hover card is intended for mouse users only so will not respond to keyboard navigation. The trigger renders as a standard link and the content is purely supplementary — it is not required to make sense of the page."</p>
    }
}
