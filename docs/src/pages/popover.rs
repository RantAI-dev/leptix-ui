use leptix_ui::popover::*;
use leptos::prelude::*;

#[component]
pub fn PopoverPage() -> impl IntoView {
    view! {
        <h1>"Popover"</h1>
        <p class="description">
            "Displays rich content in a portal, triggered by a button."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="popover">
            <div class="hero-demo-card">
            <div class="demo-popover">
                <Popover>
                    <PopoverTrigger>"Open Popover"</PopoverTrigger>
                    <PopoverPortal>
                        <PopoverContent>
                            <p>"This is the popover content. You can place any rich content here."</p>
                            <PopoverArrow />
                        </PopoverContent>
                    </PopoverPortal>
                </Popover>
            </div>
            </div>
        </div>
        <div class="code-block">
            <div class="code-block-header">
                <span class="code-block-lang">"styles.css"</span>
            </div>
            <pre>{".PopoverContent {\n  border-radius: 4px;\n  padding: 20px;\n  width: 260px;\n  font-size: 14px;\n  line-height: 1.5;\n  background-color: white;\n  box-shadow: hsl(206 22% 7% / 35%) 0px 10px 38px -10px,\n    hsl(206 22% 7% / 20%) 0px 10px 20px -15px;\n  animation: scaleIn 200ms ease;\n}\n.PopoverContent:focus {\n  outline: none;\n  box-shadow: hsl(206 22% 7% / 35%) 0px 10px 38px -10px,\n    hsl(206 22% 7% / 20%) 0px 10px 20px -15px,\n    0 0 0 2px var(--violet-7);\n}\n\n@keyframes scaleIn {\n  from { opacity: 0; transform: scale(0.96); }\n  to { opacity: 1; transform: scale(1); }\n}"}</pre>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Customizable side, alignment, offsets."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Optionally render a pointing arrow."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Focus is fully managed and customizable."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Dismissing and layering behavior is highly customizable."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-popover"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_popover::*;\n\nview! {\n    <Popover>\n        <PopoverTrigger />\n        <PopoverPortal>\n            <PopoverContent>\n                <PopoverArrow />\n            </PopoverContent>\n        </PopoverPortal>\n    </Popover>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a popover."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state of the popover."</td></tr>
                    <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"The open state of the popover when it is initially rendered."</td></tr>
                    <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the open state of the popover changes."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that toggles the popover."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
            </tbody>
        </table>

        <h3 id="portal">"Portal"</h3>
        <p>"When used, portals the content part into the body."</p>

        <h3 id="content">"Content"</h3>
        <p>"The component that pops out when the popover is open."</p>
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
        <p>"An optional arrow element to render alongside the popover."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal" style="color:var(--text-link)">"Dialog WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Opens/closes the popover."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Opens/closes the popover."</td></tr>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus to the next focusable element within the content. When the last one is reached, moves focus to the next focusable element after the trigger."</td></tr>
                <tr><td><kbd>"Shift + Tab"</kbd></td><td>"Moves focus to the previous focusable element. When the first one is reached, moves focus back to the trigger."</td></tr>
                <tr><td><kbd>"Escape"</kbd></td><td>"Closes the popover and moves focus to the trigger."</td></tr>
            </tbody>
        </table>
    }
}
