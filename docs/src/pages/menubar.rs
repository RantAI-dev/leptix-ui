use crate::HeroCodeBlock;
use leptix_ui::menubar::*;
use leptos::prelude::*;

#[component]
pub fn MenubarPage() -> impl IntoView {
    view! {
        <h1>"Menubar"</h1>
        <p class="description">
            "A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="menubar">
            <div class="hero-demo-card">
            <div class="demo-menubar">
                <Menubar>
                    <MenubarMenu>
                        <MenubarTrigger>"File"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent>
                                <MenubarItem>"New Tab"</MenubarItem>
                                <MenubarItem>"New Window"</MenubarItem>
                                <MenubarSeparator />
                                <MenubarItem>"Print"</MenubarItem>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>
                    <MenubarMenu>
                        <MenubarTrigger>"Edit"</MenubarTrigger>
                        <MenubarPortal>
                            <MenubarContent>
                                <MenubarItem>"Undo"</MenubarItem>
                                <MenubarItem>"Redo"</MenubarItem>
                                <MenubarSeparator />
                                <MenubarItem>"Cut"</MenubarItem>
                                <MenubarItem>"Copy"</MenubarItem>
                                <MenubarItem>"Paste"</MenubarItem>
                            </MenubarContent>
                        </MenubarPortal>
                    </MenubarMenu>
                </Menubar>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_ui::menubar::*;\n\nview! {\n    <Menubar attr:class=\"MenubarRoot\">\n        <MenubarMenu>\n            <MenubarTrigger attr:class=\"MenubarTrigger\">\"File\"</MenubarTrigger>\n            <MenubarPortal>\n                <MenubarContent attr:class=\"MenubarContent\">\n                    <MenubarItem attr:class=\"MenubarItem\">\"New Tab\"</MenubarItem>\n                    <MenubarItem attr:class=\"MenubarItem\">\"New Window\"</MenubarItem>\n                    <MenubarSeparator attr:class=\"MenubarSeparator\" />\n                    <MenubarItem attr:class=\"MenubarItem\">\"Print\"</MenubarItem>\n                </MenubarContent>\n            </MenubarPortal>\n        </MenubarMenu>\n    </Menubar>\n}"
            css_styles=".MenubarRoot {\n  display: flex;\n  background-color: white;\n  padding: 3px;\n  border-radius: 6px;\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n\n.MenubarTrigger {\n  padding: 8px 12px;\n  outline: none;\n  user-select: none;\n  font-weight: 500;\n  line-height: 1;\n  border-radius: 4px;\n  color: var(--violet-11);\n  font-size: 13px;\n  display: flex;\n  align-items: center;\n  justify-content: space-between;\n  gap: 2px;\n}\n\n.MenubarTrigger[data-highlighted],\n.MenubarTrigger[data-state=\"open\"] {\n  background-color: var(--violet-4);\n}\n\n.MenubarContent {\n  min-width: 180px;\n  background-color: white;\n  border-radius: 6px;\n  padding: 5px;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.MenubarItem {\n  font-size: 13px;\n  line-height: 1;\n  color: var(--violet-11);\n  border-radius: 3px;\n  display: flex;\n  align-items: center;\n  height: 25px;\n  padding: 0 10px;\n  position: relative;\n  user-select: none;\n  outline: none;\n}\n\n.MenubarItem[data-highlighted] {\n  background-color: var(--violet-9);\n  color: var(--violet-1);\n}\n\n.MenubarSeparator {\n  height: 1px;\n  background-color: var(--violet-6);\n  margin: 5px;\n}"
            modules_usage="use leptix_ui::menubar::*;\n// use styles from module\n\nview! {\n    <Menubar attr:class=styles.root>\n        <MenubarMenu>\n            <MenubarTrigger attr:class=styles.trigger>\"File\"</MenubarTrigger>\n            <MenubarPortal>\n                <MenubarContent attr:class=styles.content>\n                    <MenubarItem attr:class=styles.item>\"New Tab\"</MenubarItem>\n                    <MenubarItem attr:class=styles.item>\"New Window\"</MenubarItem>\n                    <MenubarSeparator attr:class=styles.separator />\n                    <MenubarItem attr:class=styles.item>\"Print\"</MenubarItem>\n                </MenubarContent>\n            </MenubarPortal>\n        </MenubarMenu>\n    </Menubar>\n}"
            modules_styles=".root {\n  display: flex;\n  background-color: white;\n  padding: 3px;\n  border-radius: 6px;\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n\n.trigger {\n  padding: 8px 12px;\n  outline: none;\n  user-select: none;\n  font-weight: 500;\n  line-height: 1;\n  border-radius: 4px;\n  color: var(--violet-11);\n  font-size: 13px;\n  display: flex;\n  align-items: center;\n  justify-content: space-between;\n  gap: 2px;\n}\n\n.trigger[data-highlighted],\n.trigger[data-state=\"open\"] {\n  background-color: var(--violet-4);\n}\n\n.content {\n  min-width: 180px;\n  background-color: white;\n  border-radius: 6px;\n  padding: 5px;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.item {\n  font-size: 13px;\n  line-height: 1;\n  color: var(--violet-11);\n  border-radius: 3px;\n  display: flex;\n  align-items: center;\n  height: 25px;\n  padding: 0 10px;\n  position: relative;\n  user-select: none;\n  outline: none;\n}\n\n.item[data-highlighted] {\n  background-color: var(--violet-9);\n  color: var(--violet-1);\n}\n\n.separator {\n  height: 1px;\n  background-color: var(--violet-6);\n  margin: 5px;\n}"
            tailwind_usage="use leptix_ui::menubar::*;\n\nview! {\n    <Menubar attr:class=\"flex bg-white p-0.5 rounded-md shadow-md\">\n        <MenubarMenu>\n            <MenubarTrigger attr:class=\"py-2 px-3 outline-none select-none font-medium rounded text-violet-700 text-sm flex items-center gap-0.5 data-[highlighted]:bg-violet-100 data-[state=open]:bg-violet-100\">\n                \"File\"\n            </MenubarTrigger>\n            <MenubarPortal>\n                <MenubarContent attr:class=\"min-w-[180px] bg-white rounded-md p-1 shadow-lg\">\n                    <MenubarItem attr:class=\"text-sm text-violet-700 rounded flex items-center h-[25px] px-2.5 select-none outline-none data-[highlighted]:bg-violet-500 data-[highlighted]:text-white\">\n                        \"New Tab\"\n                    </MenubarItem>\n                    <MenubarSeparator attr:class=\"h-px bg-violet-200 m-1\" />\n                    <MenubarItem attr:class=\"text-sm text-violet-700 rounded flex items-center h-[25px] px-2.5 select-none outline-none data-[highlighted]:bg-violet-500 data-[highlighted]:text-white\">\n                        \"Print\"\n                    </MenubarItem>\n                </MenubarContent>\n            </MenubarPortal>\n        </MenubarMenu>\n    </Menubar>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      minWidth: {\n        menu: \"180px\",\n      },\n    },\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports submenus with configurable reading direction."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports items, labels, groups of items."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports checkable items (single or multiple) via checkbox or radio items."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-menubar"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_menubar::*;\n\nview! {\n    <Menubar>\n        <MenubarMenu>\n            <MenubarTrigger />\n            <MenubarPortal>\n                <MenubarContent>\n                    <MenubarLabel />\n                    <MenubarItem />\n                    <MenubarCheckboxItem />\n                    <MenubarRadioGroup>\n                        <MenubarRadioItem />\n                    </MenubarRadioGroup>\n                    <MenubarSub>\n                        <MenubarSubTrigger />\n                        <MenubarSubContent />\n                    </MenubarSub>\n                    <MenubarSeparator />\n                    <MenubarItemIndicator />\n                </MenubarContent>\n            </MenubarPortal>\n        </MenubarMenu>\n    </Menubar>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a menubar."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The controlled value of the menu to open."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<String>"</td><td>"The value of the menu that should be open when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<String>"</td><td>"Event handler called when the value changes."</td></tr>
                    <tr><td>"dir"</td><td>"MaybeProp<String>"</td><td>"The reading direction. Default: \"ltr\"."</td></tr>
                    <tr><td>"r#loop"</td><td>"MaybeProp<bool>"</td><td>"When true, keyboard navigation will loop from last item to first, and vice versa. Default: true."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="menu">"Menu"</h3>
        <p>"A top level menu item, contains a trigger with content combination."</p>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that toggles the content. By default, the "<code>"MenubarContent"</code>" will position itself against the trigger."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-highlighted]"</td><td>"Present when highlighted"</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        <h3 id="content">"Content"</h3>
        <p>"The component that pops out when a menu is open."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"side"</td><td>"MaybeProp<String>"</td><td>"The preferred side of the trigger to render against. Default: \"bottom\"."</td></tr>
                    <tr><td>"side_offset"</td><td>"MaybeProp<f64>"</td><td>"The distance in pixels from the trigger."</td></tr>
                    <tr><td>"align"</td><td>"MaybeProp<String>"</td><td>"The preferred alignment against the trigger. Default: \"start\"."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="item">"Item"</h3>
        <p>"The component that contains the menubar items."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents interaction."</td></tr>
                    <tr><td>"on_select"</td><td>"Callback<()>"</td><td>"Event handler called when the user selects an item."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="checkbox-item">"CheckboxItem"</h3>
        <p>"An item that can be controlled and rendered like a checkbox."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"checked"</td><td>"MaybeProp<bool>"</td><td>"The controlled checked state."</td></tr>
                    <tr><td>"on_checked_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the checked state changes."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="radio-group">"RadioGroup"</h3>
        <p>"Used to group multiple radio items."</p>

        <h3 id="radio-item">"RadioItem"</h3>
        <p>"An item that can be controlled and rendered like a radio button."</p>

        <h3 id="item-indicator">"ItemIndicator"</h3>
        <p>"Renders when the parent CheckboxItem or RadioItem is checked."</p>

        <h3 id="sub">"Sub"</h3>
        <p>"Contains all parts of a submenu."</p>

        <h3 id="sub-trigger">"SubTrigger"</h3>
        <p>"An item that opens a submenu."</p>

        <h3 id="sub-content">"SubContent"</h3>
        <p>"The floating content that pops out when a submenu is open."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/menubar" style="color:var(--text-link)">"Menu bar WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Activates the focused item."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Activates the focused item."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Opens the menu or moves focus to the next item."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Moves focus to the previous item."</td></tr>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Opens the next menu or moves focus to the next trigger."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Opens the previous menu or moves focus to the previous trigger."</td></tr>
                <tr><td><kbd>"Escape"</kbd></td><td>"Closes the current menu and moves focus to its trigger."</td></tr>
            </tbody>
        </table>
    }
}
