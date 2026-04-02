use crate::HeroCodeBlock;
use leptix_ui::select::*;
use leptos::prelude::*;

#[component]
pub fn SelectPage() -> impl IntoView {
    view! {
        <h1>"Select"</h1>
        <p class="description">
            "Displays a list of options for the user to pick from, triggered by a button."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="select">
            <div class="hero-demo-card">
            <div class="demo-select">
                <Select>
                    <SelectTrigger>
                        <SelectValue placeholder="Select a fruit" />
                    </SelectTrigger>
                    <SelectPortal>
                        <SelectContent>
                            <SelectViewport>
                                <SelectItem value="apple">
                                    <SelectItemText>"Apple"</SelectItemText>
                                </SelectItem>
                                <SelectItem value="banana">
                                    <SelectItemText>"Banana"</SelectItemText>
                                </SelectItem>
                                <SelectItem value="cherry">
                                    <SelectItemText>"Cherry"</SelectItemText>
                                </SelectItem>
                            </SelectViewport>
                        </SelectContent>
                    </SelectPortal>
                </Select>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_ui::select::*;\n\nview! {\n    <Select>\n        <SelectTrigger attr:class=\"SelectTrigger\">\n            <SelectValue placeholder=\"Select a fruit\" />\n        </SelectTrigger>\n        <SelectPortal>\n            <SelectContent attr:class=\"SelectContent\">\n                <SelectViewport attr:class=\"SelectViewport\">\n                    <SelectItem value=\"apple\" attr:class=\"SelectItem\">\n                        <SelectItemText>\"Apple\"</SelectItemText>\n                    </SelectItem>\n                    <SelectItem value=\"banana\" attr:class=\"SelectItem\">\n                        <SelectItemText>\"Banana\"</SelectItemText>\n                    </SelectItem>\n                </SelectViewport>\n            </SelectContent>\n        </SelectPortal>\n    </Select>\n}"
            css_styles=".SelectTrigger {\n  display: inline-flex;\n  align-items: center;\n  justify-content: center;\n  border-radius: 4px;\n  padding: 0 15px;\n  font-size: 13px;\n  line-height: 1;\n  height: 35px;\n  gap: 5px;\n  background-color: white;\n  color: var(--violet-11);\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n\n.SelectTrigger:hover {\n  background-color: var(--violet-3);\n}\n\n.SelectContent {\n  overflow: hidden;\n  background-color: white;\n  border-radius: 6px;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.SelectViewport {\n  padding: 5px;\n}\n\n.SelectItem {\n  font-size: 13px;\n  line-height: 1;\n  color: var(--violet-11);\n  border-radius: 3px;\n  display: flex;\n  align-items: center;\n  height: 25px;\n  padding: 0 35px 0 25px;\n  position: relative;\n  user-select: none;\n  outline: none;\n}\n\n.SelectItem[data-highlighted] {\n  background-color: var(--violet-9);\n  color: var(--violet-1);\n}"
            modules_usage="use leptix_ui::select::*;\n// use styles from module\n\nview! {\n    <Select>\n        <SelectTrigger attr:class=styles.trigger>\n            <SelectValue placeholder=\"Select a fruit\" />\n        </SelectTrigger>\n        <SelectPortal>\n            <SelectContent attr:class=styles.content>\n                <SelectViewport attr:class=styles.viewport>\n                    <SelectItem value=\"apple\" attr:class=styles.item>\n                        <SelectItemText>\"Apple\"</SelectItemText>\n                    </SelectItem>\n                    <SelectItem value=\"banana\" attr:class=styles.item>\n                        <SelectItemText>\"Banana\"</SelectItemText>\n                    </SelectItem>\n                </SelectViewport>\n            </SelectContent>\n        </SelectPortal>\n    </Select>\n}"
            modules_styles=".trigger {\n  display: inline-flex;\n  align-items: center;\n  justify-content: center;\n  border-radius: 4px;\n  padding: 0 15px;\n  font-size: 13px;\n  line-height: 1;\n  height: 35px;\n  gap: 5px;\n  background-color: white;\n  color: var(--violet-11);\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n\n.trigger:hover {\n  background-color: var(--violet-3);\n}\n\n.content {\n  overflow: hidden;\n  background-color: white;\n  border-radius: 6px;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.viewport {\n  padding: 5px;\n}\n\n.item {\n  font-size: 13px;\n  line-height: 1;\n  color: var(--violet-11);\n  border-radius: 3px;\n  display: flex;\n  align-items: center;\n  height: 25px;\n  padding: 0 35px 0 25px;\n  position: relative;\n  user-select: none;\n  outline: none;\n}\n\n.item[data-highlighted] {\n  background-color: var(--violet-9);\n  color: var(--violet-1);\n}"
            tailwind_usage="use leptix_ui::select::*;\n\nview! {\n    <Select>\n        <SelectTrigger attr:class=\"inline-flex items-center justify-center rounded px-4 text-sm h-[35px] gap-1 bg-white text-violet-700 shadow-md hover:bg-violet-100\">\n            <SelectValue placeholder=\"Select a fruit\" />\n        </SelectTrigger>\n        <SelectPortal>\n            <SelectContent attr:class=\"overflow-hidden bg-white rounded-md shadow-lg\">\n                <SelectViewport attr:class=\"p-1\">\n                    <SelectItem value=\"apple\"\n                        attr:class=\"text-sm text-violet-700 rounded flex items-center h-[25px] pr-9 pl-6 select-none outline-none data-[highlighted]:bg-violet-500 data-[highlighted]:text-white\">\n                        <SelectItemText>\"Apple\"</SelectItemText>\n                    </SelectItem>\n                </SelectViewport>\n            </SelectContent>\n        </SelectPortal>\n    </Select>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      minWidth: {\n        select: \"180px\",\n      },\n    },\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports items, labels, groups of items."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Focus is fully managed."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Typeahead support."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-select"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_select::*;\n\nview! {\n    <Select>\n        <SelectTrigger>\n            <SelectValue placeholder=\"Pick one\" />\n            <SelectIcon />\n        </SelectTrigger>\n        <SelectPortal>\n            <SelectContent>\n                <SelectScrollUpButton />\n                <SelectViewport>\n                    <SelectGroup>\n                        <SelectLabel />\n                        <SelectItem value=\"…\">\n                            <SelectItemText />\n                            <SelectItemIndicator />\n                        </SelectItem>\n                    </SelectGroup>\n                    <SelectSeparator />\n                </SelectViewport>\n                <SelectScrollDownButton />\n            </SelectContent>\n        </SelectPortal>\n    </Select>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a select."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The controlled value."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<String>"</td><td>"The value when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<String>"</td><td>"Event handler called when the value changes."</td></tr>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents interaction."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that toggles the select. The SelectContent will position itself relative to the trigger."</p>

        <h3 id="value">"Value"</h3>
        <p>"The part that reflects the selected value."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"placeholder"</td><td>"&str"</td><td>"Text shown when no value is selected."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="item">"Item"</h3>
        <p>"A selectable item in the list."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"String"</td><td>"The value given as data when submitted with a name."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the item."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="item-indicator">"ItemIndicator"</h3>
        <p>"Renders when the parent item is selected. Automatically hidden when the item is not selected."</p>

        <h3 id="scroll-up-button">"ScrollUpButton"</h3>
        <p>"An optional button used to scroll up in the viewport. Automatically shown when the viewport can scroll up and hidden when it cannot. Scrolls on pointer hold."</p>

        <h3 id="scroll-down-button">"ScrollDownButton"</h3>
        <p>"An optional button used to scroll down in the viewport. Automatically shown when the viewport can scroll down and hidden when it cannot. Scrolls on pointer hold."</p>

        <h3 id="group">"Group"</h3>
        <p>"Used to group multiple items. Use in conjunction with SelectLabel for accessibility."</p>

        <h3 id="label">"Label"</h3>
        <p>"Used to render the label of a group."</p>

        <h3 id="separator">"Separator"</h3>
        <p>"Used to visually separate items in the select."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/listbox" style="color:var(--text-link)">"ListBox WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"When focus is on trigger, opens the select and focuses the selected or first item. When focus is on an item, selects it."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"When focus is on trigger, opens the select. When focus is on an item, selects it."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"When focus is on trigger, opens the select. When open, moves focus to the next item."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"When focus is on trigger, opens the select. When open, moves focus to the previous item."</td></tr>
                <tr><td><kbd>"Escape"</kbd></td><td>"Closes the select and moves focus back to the trigger."</td></tr>
            </tbody>
        </table>
    }
}
