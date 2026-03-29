use crate::HeroCodeBlock;
use leptix_ui::tooltip::*;
use leptos::prelude::*;

#[component]
pub fn TooltipPage() -> impl IntoView {
    view! {
        <h1>"Tooltip"</h1>
        <p class="description">
            "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="tooltip">
            <div class="hero-demo-card">
            <div class="demo-tooltip">
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger>"Hover me"</TooltipTrigger>
                        <TooltipPortal>
                            <TooltipContent>
                                "Tooltip text"
                                <TooltipArrow />
                            </TooltipContent>
                        </TooltipPortal>
                    </Tooltip>
                </TooltipProvider>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_tooltip::*;\n\nview! {\n    <TooltipProvider>\n        <Tooltip>\n            <TooltipTrigger attr:class=\"TooltipTrigger\">\"Hover me\"</TooltipTrigger>\n            <TooltipPortal>\n                <TooltipContent attr:class=\"TooltipContent\">\n                    \"Tooltip text\"\n                    <TooltipArrow />\n                </TooltipContent>\n            </TooltipPortal>\n        </Tooltip>\n    </TooltipProvider>\n}"
            css_styles=".TooltipContent {\n  border-radius: 4px;\n  padding: 10px 15px;\n  font-size: 15px;\n  line-height: 1;\n  color: var(--violet-11);\n  background-color: white;\n  box-shadow: hsl(206 22% 7% / 35%) 0px 10px 38px -10px,\n    hsl(206 22% 7% / 20%) 0px 10px 20px -15px;\n  user-select: none;\n  animation: fadeIn 200ms ease;\n}\n\n@keyframes fadeIn {\n  from { opacity: 0; }\n  to { opacity: 1; }\n}"
            modules_usage="use leptix_tooltip::*;\n// use styles from module\n\nview! {\n    <TooltipProvider>\n        <Tooltip>\n            <TooltipTrigger attr:class=styles.trigger>\"Hover me\"</TooltipTrigger>\n            <TooltipPortal>\n                <TooltipContent attr:class=styles.content>\n                    \"Tooltip text\"\n                    <TooltipArrow />\n                </TooltipContent>\n            </TooltipPortal>\n        </Tooltip>\n    </TooltipProvider>\n}"
            modules_styles=".content {\n  border-radius: 4px;\n  padding: 10px 15px;\n  font-size: 15px;\n  line-height: 1;\n  color: var(--violet-11);\n  background-color: white;\n  box-shadow: hsl(206 22% 7% / 35%) 0px 10px 38px -10px,\n    hsl(206 22% 7% / 20%) 0px 10px 20px -15px;\n  user-select: none;\n  animation: fadeIn 200ms ease;\n}\n\n@keyframes fadeIn {\n  from { opacity: 0; }\n  to { opacity: 1; }\n}"
            tailwind_usage="use leptix_tooltip::*;\n\nview! {\n    <TooltipProvider>\n        <Tooltip>\n            <TooltipTrigger>\"Hover me\"</TooltipTrigger>\n            <TooltipPortal>\n                <TooltipContent attr:class=\"rounded px-4 py-2.5 text-[15px] leading-none text-violet-700 bg-white shadow-lg select-none animate-fadeIn\">\n                    \"Tooltip text\"\n                    <TooltipArrow />\n                </TooltipContent>\n            </TooltipPortal>\n        </Tooltip>\n    </TooltipProvider>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      keyframes: {\n        fadeIn: {\n          from: { opacity: \"0\" },\n          to: { opacity: \"1\" },\n        },\n      },\n      animation: {\n        fadeIn: \"fadeIn 200ms ease\",\n      },\n    },\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Opens when the trigger is focused or hovered."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Closes when the trigger is activated or when pressing Escape."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports custom timings."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Provider to control display delay globally."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-tooltip"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_tooltip::*;\n\nview! {\n    <TooltipProvider>\n        <Tooltip>\n            <TooltipTrigger />\n            <TooltipPortal>\n                <TooltipContent>\n                    <TooltipArrow />\n                </TooltipContent>\n            </TooltipPortal>\n        </Tooltip>\n    </TooltipProvider>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="provider">"Provider"</h3>
        <p>"Wraps your app to provide global tooltip functionality."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"delay_duration"</td><td>"MaybeProp<i32>"</td><td>"The duration from when the mouse enters a tooltip trigger until the tooltip opens. Default: 700ms."</td></tr>
                    <tr><td>"skip_delay_duration"</td><td>"MaybeProp<i32>"</td><td>"How much time a user has to enter another trigger without incurring a delay again. Default: 300ms."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a tooltip."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state of the tooltip."</td></tr>
                    <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"The open state of the tooltip when it is initially rendered."</td></tr>
                    <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the open state of the tooltip changes."</td></tr>
                    <tr><td>"delay_duration"</td><td>"MaybeProp<i32>"</td><td>"Override the duration given to the Provider for this specific tooltip. Default: 700ms."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that toggles the tooltip. By default, the tooltip content will position itself against the trigger."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"closed\" | \"delayed-open\" | \"instant-open\""</td></tr>
            </tbody>
        </table>

        <h3 id="portal">"Portal"</h3>
        <p>"When used, portals the content part into the body."</p>

        <h3 id="content">"Content"</h3>
        <p>"The component that pops out when the tooltip is open."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"side"</td><td>"MaybeProp<String>"</td><td>"The preferred side of the trigger to render against. Default: \"top\"."</td></tr>
                    <tr><td>"side_offset"</td><td>"MaybeProp<f64>"</td><td>"The distance in pixels from the trigger."</td></tr>
                    <tr><td>"align"</td><td>"MaybeProp<String>"</td><td>"The preferred alignment against the trigger. Default: \"center\"."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"closed\" | \"delayed-open\" | \"instant-open\""</td></tr>
                <tr><td>"[data-side]"</td><td>"\"top\" | \"right\" | \"bottom\" | \"left\""</td></tr>
                <tr><td>"[data-align]"</td><td>"\"start\" | \"center\" | \"end\""</td></tr>
            </tbody>
        </table>

        <h3 id="arrow">"Arrow"</h3>
        <p>"An optional arrow element to render alongside the tooltip."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/tooltip" style="color:var(--text-link)">"Tooltip WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Tab"</kbd></td><td>"Opens/closes the tooltip without delay."</td></tr>
                <tr><td><kbd>"Space"</kbd></td><td>"If open, closes the tooltip without delay."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"If open, closes the tooltip without delay."</td></tr>
                <tr><td><kbd>"Escape"</kbd></td><td>"If open, closes the tooltip without delay."</td></tr>
            </tbody>
        </table>
    }
}
