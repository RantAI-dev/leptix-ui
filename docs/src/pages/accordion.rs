use crate::HeroCodeBlock;
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
            <div class="demo-accordion">
                <Accordion r#type=AccordionType::Single collapsible=true default_value=vec!["item-1".to_string()]>
                    <AccordionItem value="item-1">
                        <AccordionTrigger>
                            "Is it accessible?"
                            <span class="demo-accordion-chevron">"▾"</span>
                        </AccordionTrigger>
                        <AccordionContent>
                            "Yes. It adheres to the WAI-ARIA design pattern."
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="item-2">
                        <AccordionTrigger>
                            "Is it unstyled?"
                            <span class="demo-accordion-chevron">"▾"</span>
                        </AccordionTrigger>
                        <AccordionContent>
                            "Yes. It ships with zero styles so you have full control over the look and feel."
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="item-3">
                        <AccordionTrigger>
                            "Can it be animated?"
                            <span class="demo-accordion-chevron">"▾"</span>
                        </AccordionTrigger>
                        <AccordionContent>
                            "Yes. You can animate the open and close transitions using CSS or a Leptos animation library."
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_ui::accordion::*;\n\nview! {\n    <Accordion attr:class=\"AccordionRoot\" r#type=AccordionType::Single collapsible=true default_value=vec![\"item-1\".to_string()]>\n        <AccordionItem attr:class=\"AccordionItem\" value=\"item-1\">\n            <AccordionTrigger attr:class=\"AccordionTrigger\">\"Is it accessible?\"</AccordionTrigger>\n            <AccordionContent attr:class=\"AccordionContent\">\"Yes. It adheres to the WAI-ARIA design pattern.\"</AccordionContent>\n        </AccordionItem>\n    </Accordion>\n}"
            css_styles=".AccordionRoot {\n  border-radius: 6px;\n  width: 300px;\n  background-color: var(--mauve-6);\n  box-shadow: 0 2px 10px var(--black-a4);\n}\n\n.AccordionItem {\n  overflow: hidden;\n  margin-top: 1px;\n}\n.AccordionItem:first-child {\n  margin-top: 0;\n  border-top-left-radius: 4px;\n  border-top-right-radius: 4px;\n}\n.AccordionItem:last-child {\n  border-bottom-left-radius: 4px;\n  border-bottom-right-radius: 4px;\n}\n\n.AccordionTrigger {\n  font-family: inherit;\n  padding: 0 20px;\n  height: 45px;\n  flex: 1;\n  display: flex;\n  align-items: center;\n  justify-content: space-between;\n  font-size: 15px;\n  line-height: 1;\n  background-color: white;\n  cursor: pointer;\n}\n.AccordionTrigger[data-state=\"open\"] {\n  background-color: var(--mauve-2);\n}\n\n.AccordionContent {\n  overflow: hidden;\n  font-size: 15px;\n  color: var(--mauve-11);\n  background-color: var(--mauve-2);\n  padding: 15px 20px;\n}\n.AccordionContent[data-state=\"closed\"] {\n  display: none;\n}"
            modules_usage="// import styles from \"./accordion.module.css\";\nuse leptix_ui::accordion::*;\n\nview! {\n    <Accordion attr:class=styles.root r#type=AccordionType::Single collapsible=true default_value=vec![\"item-1\".to_string()]>\n        <AccordionItem attr:class=styles.item value=\"item-1\">\n            <AccordionTrigger attr:class=styles.trigger>\"Is it accessible?\"</AccordionTrigger>\n            <AccordionContent attr:class=styles.content>\"Yes. It adheres to the WAI-ARIA design pattern.\"</AccordionContent>\n        </AccordionItem>\n    </Accordion>\n}"
            modules_styles=".root {\n  border-radius: 6px;\n  width: 300px;\n  background-color: var(--mauve-6);\n  box-shadow: 0 2px 10px var(--black-a4);\n}\n\n.item {\n  overflow: hidden;\n  margin-top: 1px;\n}\n.item:first-child {\n  margin-top: 0;\n  border-top-left-radius: 4px;\n  border-top-right-radius: 4px;\n}\n.item:last-child {\n  border-bottom-left-radius: 4px;\n  border-bottom-right-radius: 4px;\n}\n\n.trigger {\n  font-family: inherit;\n  padding: 0 20px;\n  height: 45px;\n  flex: 1;\n  display: flex;\n  align-items: center;\n  justify-content: space-between;\n  font-size: 15px;\n  line-height: 1;\n  background-color: white;\n  cursor: pointer;\n}\n.trigger[data-state=\"open\"] {\n  background-color: var(--mauve-2);\n}\n\n.content {\n  overflow: hidden;\n  font-size: 15px;\n  color: var(--mauve-11);\n  background-color: var(--mauve-2);\n  padding: 15px 20px;\n}\n.content[data-state=\"closed\"] {\n  display: none;\n}"
            tailwind_usage="use leptix_ui::accordion::*;\n\nview! {\n    <Accordion attr:class=\"rounded-md w-[300px] bg-gray-200 shadow-md\" r#type=AccordionType::Single collapsible=true default_value=vec![\"item-1\".to_string()]>\n        <AccordionItem attr:class=\"overflow-hidden mt-px first:mt-0 first:rounded-t last:rounded-b\" value=\"item-1\">\n            <AccordionTrigger attr:class=\"font-inherit px-5 h-[45px] flex-1 flex items-center justify-between text-sm bg-white cursor-pointer data-[state=open]:bg-gray-50\">\n                \"Is it accessible?\"\n            </AccordionTrigger>\n            <AccordionContent attr:class=\"overflow-hidden text-sm text-gray-500 bg-gray-50 px-5 py-4 data-[state=closed]:hidden\">\n                \"Yes. It adheres to the WAI-ARIA design pattern.\"\n            </AccordionContent>\n        </AccordionItem>\n    </Accordion>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      colors: {\n        accordion: {\n          bg: \"var(--mauve-6)\",\n          content: \"var(--mauve-2)\",\n          text: \"var(--mauve-11)\",\n        },\n      },\n      boxShadow: {\n        accordion: \"0 2px 10px var(--black-a4)\",\n      },\n      borderRadius: {\n        accordion: \"6px\",\n      },\n    },\n  },\n} satisfies Config;"
        />

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
