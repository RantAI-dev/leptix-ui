use crate::HeroCodeBlock;
use leptix_ui::context_menu::*;
use leptos::prelude::*;

#[component]
pub fn ContextMenuPage() -> impl IntoView {
    view! {
        <h1>"Context Menu"</h1>
        <p class="description">
            "Displays a menu located at the pointer, triggered by a right-click or a long-press."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="context-menu">
            <div class="hero-demo-card">
            <div class="demo-context-menu">
                <ContextMenu>
                    <ContextMenuTrigger>
                        <div style="border:2px dashed var(--border);border-radius:8px;padding:3rem 2rem;text-align:center;color:var(--text-secondary);user-select:none;">
                            "Right click here"
                        </div>
                    </ContextMenuTrigger>
                    <ContextMenuPortal>
                        <ContextMenuContent>
                            <ContextMenuItem>"Back"</ContextMenuItem>
                            <ContextMenuItem>"Forward"</ContextMenuItem>
                            <ContextMenuItem>"Reload"</ContextMenuItem>
                            <ContextMenuSeparator />
                            <ContextMenuItem>"View Source"</ContextMenuItem>
                            <ContextMenuItem>"Inspect"</ContextMenuItem>
                        </ContextMenuContent>
                    </ContextMenuPortal>
                </ContextMenu>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_ui::context_menu::*;\n\nview! {\n    <ContextMenu>\n        <ContextMenuTrigger>\n            <div>\"Right click here\"</div>\n        </ContextMenuTrigger>\n        <ContextMenuPortal>\n            <ContextMenuContent attr:class=\"ContextMenuContent\">\n                <ContextMenuItem attr:class=\"ContextMenuItem\">\"Back\"</ContextMenuItem>\n                <ContextMenuItem attr:class=\"ContextMenuItem\">\"Forward\"</ContextMenuItem>\n                <ContextMenuSeparator attr:class=\"ContextMenuSeparator\" />\n                <ContextMenuItem attr:class=\"ContextMenuItem\">\"Inspect\"</ContextMenuItem>\n            </ContextMenuContent>\n        </ContextMenuPortal>\n    </ContextMenu>\n}"
            css_styles=".ContextMenuContent {\n  min-width: 180px;\n  background-color: white;\n  border-radius: 6px;\n  padding: 5px;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.ContextMenuItem {\n  font-size: 13px;\n  line-height: 1;\n  color: var(--violet-11);\n  border-radius: 3px;\n  display: flex;\n  align-items: center;\n  height: 25px;\n  padding: 0 5px 0 25px;\n  position: relative;\n  user-select: none;\n  outline: none;\n}\n\n.ContextMenuItem[data-highlighted] {\n  background-color: var(--violet-9);\n  color: var(--violet-1);\n}\n\n.ContextMenuSeparator {\n  height: 1px;\n  background-color: var(--violet-6);\n  margin: 5px;\n}"
            modules_usage="use leptix_ui::context_menu::*;\n// use styles from module\n\nview! {\n    <ContextMenu>\n        <ContextMenuTrigger>\n            <div>\"Right click here\"</div>\n        </ContextMenuTrigger>\n        <ContextMenuPortal>\n            <ContextMenuContent attr:class=styles.content>\n                <ContextMenuItem attr:class=styles.item>\"Back\"</ContextMenuItem>\n                <ContextMenuItem attr:class=styles.item>\"Forward\"</ContextMenuItem>\n                <ContextMenuSeparator attr:class=styles.separator />\n                <ContextMenuItem attr:class=styles.item>\"Inspect\"</ContextMenuItem>\n            </ContextMenuContent>\n        </ContextMenuPortal>\n    </ContextMenu>\n}"
            modules_styles=".content {\n  min-width: 180px;\n  background-color: white;\n  border-radius: 6px;\n  padding: 5px;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n}\n\n.item {\n  font-size: 13px;\n  line-height: 1;\n  color: var(--violet-11);\n  border-radius: 3px;\n  display: flex;\n  align-items: center;\n  height: 25px;\n  padding: 0 5px 0 25px;\n  position: relative;\n  user-select: none;\n  outline: none;\n}\n\n.item[data-highlighted] {\n  background-color: var(--violet-9);\n  color: var(--violet-1);\n}\n\n.separator {\n  height: 1px;\n  background-color: var(--violet-6);\n  margin: 5px;\n}"
            tailwind_usage="use leptix_ui::context_menu::*;\n\nview! {\n    <ContextMenu>\n        <ContextMenuTrigger>\n            <div>\"Right click here\"</div>\n        </ContextMenuTrigger>\n        <ContextMenuPortal>\n            <ContextMenuContent attr:class=\"min-w-[180px] bg-white rounded-md p-1 shadow-lg\">\n                <ContextMenuItem attr:class=\"text-sm text-violet-700 rounded flex items-center h-[25px] px-1 pl-6 select-none outline-none data-[highlighted]:bg-violet-500 data-[highlighted]:text-white\">\n                    \"Back\"\n                </ContextMenuItem>\n                <ContextMenuSeparator attr:class=\"h-px bg-violet-200 m-1\" />\n                <ContextMenuItem attr:class=\"text-sm text-violet-700 rounded flex items-center h-[25px] px-1 pl-6 select-none outline-none data-[highlighted]:bg-violet-500 data-[highlighted]:text-white\">\n                    \"Inspect\"\n                </ContextMenuItem>\n            </ContextMenuContent>\n        </ContextMenuPortal>\n    </ContextMenu>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      minWidth: {\n        menu: \"180px\",\n      },\n    },\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports submenus with configurable reading direction."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports items, labels, groups of items."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports checkable items (single or multiple)."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Opens on right-click."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-context-menu"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_context_menu::*;\n\nview! {\n    <ContextMenu>\n        <ContextMenuTrigger />\n        <ContextMenuPortal>\n            <ContextMenuContent>\n                <ContextMenuItem />\n                <ContextMenuSeparator />\n                <ContextMenuItem />\n            </ContextMenuContent>\n        </ContextMenuPortal>\n    </ContextMenu>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a context menu."</p>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The area that opens the context menu. Wraps the area that will respond to right-click."</p>

        <h3 id="portal">"Portal"</h3>
        <p>"When used, portals the content part into the body."</p>

        <h3 id="content">"Content"</h3>
        <p>"The component that pops out in an open context menu."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"side_offset"</td><td>"MaybeProp<f64>"</td><td>"The distance in pixels from the trigger."</td></tr>
                    <tr><td>"align"</td><td>"MaybeProp<Align>"</td><td>"The preferred alignment against the trigger."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="item">"Item"</h3>
        <p>"The component that contains the context menu items."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the item."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Uses roving tabindex to manage focus movement among menu items."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Activates the focused item."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Activates the focused item."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Moves focus to the next item."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Moves focus to the previous item."</td></tr>
                <tr><td><kbd>"Escape"</kbd></td><td>"Closes the context menu."</td></tr>
            </tbody>
        </table>
    }
}
