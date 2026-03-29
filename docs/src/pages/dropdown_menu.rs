use crate::HeroCodeBlock;
use leptix_ui::dropdown_menu::*;
use leptos::prelude::*;

#[component]
pub fn DropdownMenuPage() -> impl IntoView {
    view! {
        <h1>"Dropdown Menu"</h1>
        <p class="description">
            "Displays a menu to the user — such as a set of actions or functions — triggered by a button."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="dropdown-menu">
            <div class="hero-demo-card">
            <div class="demo-dropdown-menu">
                <DropdownMenu>
                    <DropdownMenuTrigger>"Options"</DropdownMenuTrigger>
                    <DropdownMenuPortal>
                        <DropdownMenuContent>
                            <DropdownMenuItem>"New Tab"</DropdownMenuItem>
                            <DropdownMenuItem>"New Window"</DropdownMenuItem>
                            <DropdownMenuItem disabled=true>"Print"</DropdownMenuItem>
                            <DropdownMenuArrow />
                        </DropdownMenuContent>
                    </DropdownMenuPortal>
                </DropdownMenu>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            usage_code="use leptix_dropdown_menu::*;\n\nview! {\n    <DropdownMenu>\n        <DropdownMenuTrigger>\"Options\"</DropdownMenuTrigger>\n        <DropdownMenuPortal>\n            <DropdownMenuContent class=\"DropdownMenuContent\">\n                <DropdownMenuItem class=\"DropdownMenuItem\">\"New Tab\"</DropdownMenuItem>\n                <DropdownMenuItem class=\"DropdownMenuItem\">\"New Window\"</DropdownMenuItem>\n                <DropdownMenuItem class=\"DropdownMenuItem\" disabled=true>\"Print\"</DropdownMenuItem>\n                <DropdownMenuArrow />\n            </DropdownMenuContent>\n        </DropdownMenuPortal>\n    </DropdownMenu>\n}"
            css_code=".DropdownMenuContent {\n  min-width: 220px;\n  background-color: white;\n  border-radius: 6px;\n  padding: 5px;\n  box-shadow: 0px 10px 38px -10px rgba(22, 23, 24, 0.35),\n    0px 10px 20px -15px rgba(22, 23, 24, 0.2);\n  animation: scaleIn 200ms ease;\n}\n\n.DropdownMenuItem {\n  font-size: 13px;\n  line-height: 1;\n  color: var(--violet-11);\n  border-radius: 3px;\n  display: flex;\n  align-items: center;\n  height: 25px;\n  padding: 0 5px 0 25px;\n  position: relative;\n  user-select: none;\n  outline: none;\n}\n.DropdownMenuItem[data-highlighted] {\n  background-color: var(--violet-9);\n  color: var(--violet-1);\n}\n.DropdownMenuItem[data-disabled] {\n  color: var(--mauve-8);\n  pointer-events: none;\n}\n\n@keyframes scaleIn {\n  from { opacity: 0; transform: scale(0.96); }\n  to { opacity: 1; transform: scale(1); }\n}"
            css_modules_code=".Content {\n  min-width: 220px;\n  background-color: white;\n  border-radius: 6px;\n  padding: 5px;\n  box-shadow: 0px 10px 38px -10px rgba(22, 23, 24, 0.35),\n    0px 10px 20px -15px rgba(22, 23, 24, 0.2);\n  animation: scaleIn 200ms ease;\n}\n\n.Item {\n  font-size: 13px;\n  line-height: 1;\n  color: var(--violet-11);\n  border-radius: 3px;\n  display: flex;\n  align-items: center;\n  height: 25px;\n  padding: 0 5px 0 25px;\n  position: relative;\n  user-select: none;\n  outline: none;\n}\n.Item[data-highlighted] {\n  background-color: var(--violet-9);\n  color: var(--violet-1);\n}\n.Item[data-disabled] {\n  color: var(--mauve-8);\n  pointer-events: none;\n}\n\n@keyframes scaleIn {\n  from { opacity: 0; transform: scale(0.96); }\n  to { opacity: 1; transform: scale(1); }\n}"
            tailwind_code="<DropdownMenu>\n    <DropdownMenuTrigger>\"Options\"</DropdownMenuTrigger>\n    <DropdownMenuPortal>\n        <DropdownMenuContent class=\"min-w-[220px] bg-white rounded-md p-1 shadow-lg animate-scaleIn\">\n            <DropdownMenuItem class=\"text-[13px] leading-none text-violet-700 rounded flex items-center h-[25px] pl-6 pr-1 relative select-none outline-none data-[highlighted]:bg-violet-500 data-[highlighted]:text-violet-50 data-[disabled]:text-gray-400 data-[disabled]:pointer-events-none\">\n                \"New Tab\"\n            </DropdownMenuItem>\n        </DropdownMenuContent>\n    </DropdownMenuPortal>\n</DropdownMenu>"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports submenus with configurable reading direction."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports items, labels, groups of items."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports checkable items (single or multiple) with optional indeterminate state."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Customize side, alignment, offsets."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Focus is fully managed."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Typeahead support."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Dismissing and layering behavior is highly customizable."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-dropdown-menu"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_dropdown_menu::*;\n\nview! {\n    <DropdownMenu>\n        <DropdownMenuTrigger />\n        <DropdownMenuPortal>\n            <DropdownMenuContent>\n                <DropdownMenuItem />\n                <DropdownMenuItem />\n            </DropdownMenuContent>\n        </DropdownMenuPortal>\n    </DropdownMenu>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a dropdown menu."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state of the dropdown menu."</td></tr>
                    <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"The open state of the dropdown menu when it is initially rendered."</td></tr>
                    <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the open state of the dropdown menu changes."</td></tr>
                    <tr><td>"dir"</td><td>"MaybeProp<Direction>"</td><td>"The reading direction of submenus when applicable."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that toggles the dropdown menu."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        <h3 id="portal">"Portal"</h3>
        <p>"When used, portals the content part into the body."</p>

        <h3 id="content">"Content"</h3>
        <p>"The component that pops out when the dropdown menu is open."</p>
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
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-side]"</td><td>"\"top\" | \"right\" | \"bottom\" | \"left\""</td></tr>
                <tr><td>"[data-align]"</td><td>"\"start\" | \"center\" | \"end\""</td></tr>
            </tbody>
        </table>

        <h3 id="item">"Item"</h3>
        <p>"The component that contains the dropdown menu items."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the item."</td></tr>
                    <tr><td>"on_select"</td><td>"Callback<()>"</td><td>"Event handler called when the user selects an item (via mouse or keyboard)."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
                <tr><td>"[data-highlighted]"</td><td>"Present when highlighted"</td></tr>
            </tbody>
        </table>

        <h3 id="arrow">"Arrow"</h3>
        <p>"An optional arrow element to render alongside the dropdown menu."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/menu-button" style="color:var(--text-link)">"Menu Button WAI-ARIA design pattern"</a>" and uses "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/menu" style="color:var(--text-link)">"roving tabindex"</a>" to manage focus movement among menu items."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"When focus is on the trigger, opens the dropdown menu. When focus is on an item, activates the focused item."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"When focus is on the trigger, opens the dropdown menu. When focus is on an item, activates the focused item."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"When focus is on the trigger, opens the dropdown menu. When focus is on an item, moves focus to the next item."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"When focus is on an item, moves focus to the previous item."</td></tr>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"When focus is on a submenu trigger, opens the submenu."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"When focus is within a submenu, closes the submenu and moves focus to the parent trigger."</td></tr>
                <tr><td><kbd>"Escape"</kbd></td><td>"Closes the dropdown menu and moves focus to the trigger."</td></tr>
            </tbody>
        </table>
    }
}
