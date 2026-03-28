use leptix_ui::accordion::*;
use leptos::prelude::*;

#[component]
pub fn AccordionPage() -> impl IntoView {
    view! {
        <h1>"Accordion"</h1>
        <p class="description">
            "A vertically stacked set of interactive headings that each reveal a section of content."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="accordion">
            <div class="hero-demo-card">
            <Accordion r#type=AccordionType::Single collapsible=true default_value=vec!["item-1".to_string()] attr:class="demo-accordion">
                <AccordionItem value="item-1" attr:class="demo-accordion-item">
                    <AccordionTrigger attr:class="demo-accordion-trigger">
                        "Is it accessible?"
                        <span class="demo-accordion-chevron">"▾"</span>
                    </AccordionTrigger>
                    <AccordionContent attr:class="demo-accordion-content">
                        "Yes. It adheres to the WAI-ARIA design pattern."
                    </AccordionContent>
                </AccordionItem>
                <AccordionItem value="item-2" attr:class="demo-accordion-item">
                    <AccordionTrigger attr:class="demo-accordion-trigger">
                        "Is it unstyled?"
                        <span class="demo-accordion-chevron">"▾"</span>
                    </AccordionTrigger>
                    <AccordionContent attr:class="demo-accordion-content">
                        "Yes. It ships with zero styles so you have full control over the look and feel."
                    </AccordionContent>
                </AccordionItem>
                <AccordionItem value="item-3" attr:class="demo-accordion-item">
                    <AccordionTrigger attr:class="demo-accordion-trigger">
                        "Can it be animated?"
                        <span class="demo-accordion-chevron">"▾"</span>
                    </AccordionTrigger>
                    <AccordionContent attr:class="demo-accordion-content">
                        "Yes. You can animate the open and close transitions using CSS or a Leptos animation library."
                    </AccordionContent>
                </AccordionItem>
            </Accordion>
            </div>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports single and multiple expanded items."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span><span>"Can expand one or more items simultaneously with "<code>"AccordionType::Multiple"</code>"."</span></div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-accordion"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_accordion::*;\n\nview! {\n    <Accordion r#type=AccordionType::Single>\n        <AccordionItem value=\"item-1\">\n            <AccordionTrigger />\n            <AccordionContent />\n        </AccordionItem>\n    </Accordion>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of an accordion."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"type"</td><td>"AccordionType"</td><td>"Determines whether one or multiple items can be opened at the same time."</td></tr>
                    <tr><td>"value"</td><td>"MaybeProp<Vec<String>>"</td><td>"The controlled value of the item(s) to expand."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<Vec<String>>"</td><td>"The value of the item(s) to expand when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<Vec<String>>"</td><td>"Event handler called when the expanded state of an item changes."</td></tr>
                    <tr><td>"collapsible"</td><td>"bool"</td><td>"When type is Single, allows closing content when clicking the open trigger. Default: false."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the accordion and all its items."</td></tr>
                    <tr><td>"orientation"</td><td>"MaybeProp<String>"</td><td>"The orientation of the accordion. Default: \"vertical\"."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="item">"Item"</h3>
        <p>"Contains all the parts of a collapsible section."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"String"</td><td>"A unique value for the item."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the item."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="trigger">"Trigger"</h3>
        <p>"Toggles the collapsed state of its associated item."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="content">"Content"</h3>
        <p>"Contains the collapsible content for an item."</p>
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
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/accordion" style="color:var(--text-link)">"Accordion WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"When focus is on a trigger, toggles the collapsed state of the associated section."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"When focus is on a trigger, toggles the collapsed state of the associated section."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Moves focus to the next trigger."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Moves focus to the previous trigger."</td></tr>
                <tr><td><kbd>"Home"</kbd></td><td>"Moves focus to the first trigger."</td></tr>
                <tr><td><kbd>"End"</kbd></td><td>"Moves focus to the last trigger."</td></tr>
            </tbody>
        </table>
    }
}
