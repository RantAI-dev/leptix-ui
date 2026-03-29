use crate::HeroCodeBlock;
use leptix_ui::toolbar::*;
use leptos::prelude::*;

#[component]
pub fn ToolbarPage() -> impl IntoView {
    view! {
        <h1>"Toolbar"</h1>
        <p class="description">
            "A container for grouping a set of controls, such as buttons, toggle groups, or dropdown menus."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="toolbar">
            <div class="hero-demo-card">
            <div class="demo-toolbar">
                <Toolbar>
                    <ToolbarButton>"Undo"</ToolbarButton>
                    <ToolbarSeparator />
                    <ToolbarButton>"Redo"</ToolbarButton>
                </Toolbar>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_ui::toolbar::*;\n\nview! {\n    <Toolbar attr:class=\"ToolbarRoot\">\n        <ToolbarButton attr:class=\"ToolbarButton\">\"Undo\"</ToolbarButton>\n        <ToolbarSeparator attr:class=\"ToolbarSeparator\" />\n        <ToolbarButton attr:class=\"ToolbarButton\">\"Redo\"</ToolbarButton>\n    </Toolbar>\n}"
            css_styles=".ToolbarRoot {\n  display: flex;\n  padding: 10px;\n  width: 100%;\n  min-width: max-content;\n  border-radius: 6px;\n  background-color: white;\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n\n.ToolbarButton {\n  flex: 0 0 auto;\n  color: var(--violet-11);\n  height: 25px;\n  padding: 0 5px;\n  border-radius: 4px;\n  display: inline-flex;\n  font-size: 13px;\n  line-height: 1;\n  align-items: center;\n  justify-content: center;\n  border: none;\n  background: transparent;\n  cursor: pointer;\n}\n\n.ToolbarButton:hover {\n  background-color: var(--violet-3);\n  color: var(--violet-11);\n}\n\n.ToolbarSeparator {\n  width: 1px;\n  background-color: var(--violet-6);\n  margin: 0 10px;\n}"
            modules_usage="use leptix_ui::toolbar::*;\n// use styles from module\n\nview! {\n    <Toolbar attr:class=styles.root>\n        <ToolbarButton attr:class=styles.button>\"Undo\"</ToolbarButton>\n        <ToolbarSeparator attr:class=styles.separator />\n        <ToolbarButton attr:class=styles.button>\"Redo\"</ToolbarButton>\n    </Toolbar>\n}"
            modules_styles=".root {\n  display: flex;\n  padding: 10px;\n  width: 100%;\n  min-width: max-content;\n  border-radius: 6px;\n  background-color: white;\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n\n.button {\n  flex: 0 0 auto;\n  color: var(--violet-11);\n  height: 25px;\n  padding: 0 5px;\n  border-radius: 4px;\n  display: inline-flex;\n  font-size: 13px;\n  line-height: 1;\n  align-items: center;\n  justify-content: center;\n  border: none;\n  background: transparent;\n  cursor: pointer;\n}\n\n.button:hover {\n  background-color: var(--violet-3);\n  color: var(--violet-11);\n}\n\n.separator {\n  width: 1px;\n  background-color: var(--violet-6);\n  margin: 0 10px;\n}"
            tailwind_usage="use leptix_ui::toolbar::*;\n\nview! {\n    <Toolbar attr:class=\"flex p-2.5 w-full min-w-max rounded-md bg-white shadow-md\">\n        <ToolbarButton attr:class=\"flex-none text-violet-700 h-[25px] px-1 rounded inline-flex text-sm items-center justify-center bg-transparent cursor-pointer hover:bg-violet-100\">\n            \"Undo\"\n        </ToolbarButton>\n        <ToolbarSeparator attr:class=\"w-px bg-violet-200 mx-2.5\" />\n        <ToolbarButton attr:class=\"flex-none text-violet-700 h-[25px] px-1 rounded inline-flex text-sm items-center justify-center bg-transparent cursor-pointer hover:bg-violet-100\">\n            \"Redo\"\n        </ToolbarButton>\n    </Toolbar>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {},\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports horizontal and vertical orientations."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports toggle groups, links, buttons, and custom controls."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-toolbar"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_toolbar::*;\n\nview! {\n    <Toolbar>\n        <ToolbarButton />\n        <ToolbarSeparator />\n        <ToolbarButton />\n        <ToolbarLink />\n        <ToolbarToggleGroup r#type=\"single\">\n            <ToolbarToggleItem value=\"item\" />\n        </ToolbarToggleGroup>\n    </Toolbar>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the toolbar component parts."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"orientation"</td><td>"MaybeProp<String>"</td><td>"The orientation of the toolbar. Default: \"horizontal\"."</td></tr>
                    <tr><td>"dir"</td><td>"MaybeProp<Direction>"</td><td>"The reading direction of the toolbar."</td></tr>
                    <tr><td>"loop"</td><td>"MaybeProp<bool>"</td><td>"When true, keyboard navigation will loop from last item to first and vice versa. Default: true."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="button">"Button"</h3>
        <p>"A button item."</p>

        <h3 id="link">"Link"</h3>
        <p>"A link item."</p>

        <h3 id="separator">"Separator"</h3>
        <p>"Used to visually separate items in the toolbar."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="toggle-group">"ToggleGroup"</h3>
        <p>"A set of two-state buttons that can be toggled on or off as a group."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"type"</td><td>"MaybeProp<String>"</td><td>"Determines whether a single or multiple items can be pressed at the same time."</td></tr>
                    <tr><td>"value"</td><td>"MaybeProp<Vec<String>>"</td><td>"The controlled value of the pressed item(s)."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<Vec<String>>"</td><td>"The value of the item(s) to be pressed when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<Vec<String>>"</td><td>"Event handler called when the pressed state of an item changes."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the toggle group."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="toggle-item">"ToggleItem"</h3>
        <p>"An item in the toggle group."</p>
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
                <tr><td>"[data-state]"</td><td>"\"on\" | \"off\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/toolbar" style="color:var(--text-link)">"Toolbar WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus to the first item in the toolbar."</td></tr>
                <tr><td><kbd>"Space"</kbd></td><td>"Activates the focused button or toggle item."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Activates the focused button or toggle item."</td></tr>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Moves focus to the next item."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Moves focus to the previous item."</td></tr>
                <tr><td><kbd>"Home"</kbd></td><td>"Moves focus to the first item."</td></tr>
                <tr><td><kbd>"End"</kbd></td><td>"Moves focus to the last item."</td></tr>
            </tbody>
        </table>
    }
}
