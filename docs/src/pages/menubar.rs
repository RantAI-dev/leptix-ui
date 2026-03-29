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
        <div class="anatomy-block">{"use leptix_menubar::*;\n\nview! {\n    <Menubar>\n        <MenubarMenu>\n            <MenubarTrigger />\n            <MenubarPortal>\n                <MenubarContent>\n                    <MenubarItem />\n                    <MenubarSeparator />\n                    <MenubarItem />\n                </MenubarContent>\n            </MenubarPortal>\n        </MenubarMenu>\n    </Menubar>\n}"}</div>

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
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-highlighted]"</td><td>"Present when highlighted"</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

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
