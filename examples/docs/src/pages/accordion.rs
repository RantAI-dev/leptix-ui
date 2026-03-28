use leptix_ui::accordion::*;
use leptos::prelude::*;

#[component]
pub fn AccordionPage() -> impl IntoView {
    view! {
        <h1>"Accordion"</h1>
        <p class="subtitle">"A vertically stacked set of interactive headings that each reveal a section of content."</p>

        <div class="features">
            <span class="feature">"Single / multiple"</span>
            <span class="feature">"Collapsible"</span>
            <span class="feature">"Arrow key navigation"</span>
            <span class="feature">"Animated"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview" style="display:block;padding:24px 40px">
                <Accordion r#type=AccordionType::Single collapsible=true default_value=vec!["item-1".to_string()] attr:style="width:100%">
                    <AccordionItem value="item-1" attr:class="demo-accordion-item">
                        <AccordionTrigger attr:class="demo-accordion-trigger">
                            "Is it accessible?"
                        </AccordionTrigger>
                        <AccordionContent attr:class="demo-accordion-content">
                            "Yes. It adheres to the WAI-ARIA design pattern for accordions."
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="item-2" attr:class="demo-accordion-item">
                        <AccordionTrigger attr:class="demo-accordion-trigger">
                            "Is it unstyled?"
                        </AccordionTrigger>
                        <AccordionContent attr:class="demo-accordion-content">
                            "Yes. It ships with zero styles so you have full control over the look and feel."
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="item-3" attr:class="demo-accordion-item">
                        <AccordionTrigger attr:class="demo-accordion-trigger">
                            "Can it be animated?"
                        </AccordionTrigger>
                        <AccordionContent attr:class="demo-accordion-content">
                            "Yes. You can animate the open and close transitions using CSS or a Leptos animation library."
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            </div>
            <div class="demo-code">{"<Accordion r#type=AccordionType::Single collapsible=true>\n  <AccordionItem value=\"item-1\">\n    <AccordionTrigger>\"Is it accessible?\"</AccordionTrigger>\n    <AccordionContent>\"Yes. It adheres to WAI-ARIA...\"</AccordionContent>\n  </AccordionItem>\n  <AccordionItem value=\"item-2\">\n    <AccordionTrigger>\"Is it unstyled?\"</AccordionTrigger>\n    <AccordionContent>\"Yes. Zero styles...\"</AccordionContent>\n  </AccordionItem>\n</Accordion>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Accordion"</h3>
        <p>"Contains all the parts of an accordion."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"type"</td><td>"AccordionType"</td><td>"Single allows one item open. Multiple allows many."</td></tr>
                <tr><td>"value"</td><td>"MaybeProp<Vec<String>>"</td><td>"Controlled open items."</td></tr>
                <tr><td>"default_value"</td><td>"MaybeProp<Vec<String>>"</td><td>"Uncontrolled initial open items."</td></tr>
                <tr><td>"on_value_change"</td><td>"Callback<Vec<String>>"</td><td>"Called when open items change."</td></tr>
                <tr><td>"collapsible"</td><td>"bool"</td><td>"When Single, allows closing the open item. Default: false."</td></tr>
                <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents user interaction."</td></tr>
                <tr><td>"orientation"</td><td>"MaybeProp<String>"</td><td>"The orientation. Default: \"vertical\"."</td></tr>
            </tbody>
        </table>

        <h3>"AccordionItem"</h3>
        <p>"Contains all the parts of a collapsible section."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"value"</td><td>"String"</td><td>"A unique value for the item."</td></tr>
                <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents user interaction with this item."</td></tr>
            </tbody>
        </table>

        <h3>"AccordionTrigger"</h3>
        <p>"Toggles the collapsed state of the associated content."</p>

        <h3>"AccordionContent"</h3>
        <p>"Contains the collapsible content for an item."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"force_mount"</td><td>"MaybeProp<bool>"</td><td>"When true, content is always rendered in the DOM."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Toggles the accordion item when its trigger is focused."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Toggles the accordion item when its trigger is focused."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Moves focus to the next trigger."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Moves focus to the previous trigger."</td></tr>
                <tr><td><kbd>"Home"</kbd></td><td>"Moves focus to the first trigger."</td></tr>
                <tr><td><kbd>"End"</kbd></td><td>"Moves focus to the last trigger."</td></tr>
            </tbody>
        </table>
    }
}
