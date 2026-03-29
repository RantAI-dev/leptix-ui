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
                    <HoverCardTrigger attr:style="text-decoration:none;display:block;padding:16px 24px;border-radius:8px;border:1px solid var(--border);cursor:pointer;color:var(--accent-light);font-weight:500;font-size:15px">
                        "Hover over @leptix"
                    </HoverCardTrigger>
                    <HoverCardPortal>
                        <HoverCardContent>
                            <p>"Leptix is an open-source Rust UI component library for Leptos, inspired by Radix."</p>
                            <HoverCardArrow />
                        </HoverCardContent>
                    </HoverCardPortal>
                </HoverCard>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            usage_code="use leptix_ui::hover_card::*;\n\nview! {\n    <HoverCard>\n        <HoverCardTrigger>\"Hover over @leptix\"</HoverCardTrigger>\n        <HoverCardPortal>\n            <HoverCardContent>\n                <p>\"Preview content goes here.\"</p>\n                <HoverCardArrow />\n            </HoverCardContent>\n        </HoverCardPortal>\n    </HoverCard>\n}"
            css_code=".HoverCardContent {\n  border-radius: 6px;\n  padding: 20px;\n  width: 300px;\n  background-color: white;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.HoverCardTrigger {\n  cursor: pointer;\n  color: var(--violet-11);\n  font-weight: 500;\n}\n\n.HoverCardArrow {\n  fill: white;\n}"
            css_modules_code=".Content {\n  border-radius: 6px;\n  padding: 20px;\n  width: 300px;\n  background-color: white;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.Trigger {\n  cursor: pointer;\n  color: var(--violet-11);\n  font-weight: 500;\n}\n\n.Arrow {\n  fill: white;\n}"
            tailwind_code="view! {\n    <HoverCard>\n        <HoverCardTrigger class=\"cursor-pointer text-violet-700 font-medium\">\n            \"Hover over @leptix\"\n        </HoverCardTrigger>\n        <HoverCardPortal>\n            <HoverCardContent class=\"rounded-md p-5 w-[300px] bg-white shadow-lg\">\n                <p>\"Preview content goes here.\"</p>\n                <HoverCardArrow class=\"fill-white\" />\n            </HoverCardContent>\n        </HoverCardPortal>\n    </HoverCard>\n}"
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
