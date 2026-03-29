use leptix_ui::navigation_menu::*;
use leptos::prelude::*;

#[component]
pub fn NavigationMenuPage() -> impl IntoView {
    view! {
        <h1>"Navigation Menu"</h1>
        <p class="description">
            "A collection of links for navigating websites."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="navigation-menu">
            <div class="hero-demo-card">
            <div class="demo-navigation-menu">
                <NavigationMenu>
                    <NavigationMenuList>
                        <NavigationMenuItem>
                            <NavigationMenuLink>"Getting Started"</NavigationMenuLink>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink>"Components"</NavigationMenuLink>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink>"Documentation"</NavigationMenuLink>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </div>
            </div>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Flexible layout structure with managed tab focus."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports submenus."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-navigation-menu"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_navigation_menu::*;\n\nview! {\n    <NavigationMenu>\n        <NavigationMenuList>\n            <NavigationMenuItem>\n                <NavigationMenuTrigger />\n                <NavigationMenuContent>\n                    <NavigationMenuLink />\n                </NavigationMenuContent>\n            </NavigationMenuItem>\n\n            <NavigationMenuItem>\n                <NavigationMenuLink />\n            </NavigationMenuItem>\n        </NavigationMenuList>\n    </NavigationMenu>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a navigation menu."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The controlled value of the menu item to activate."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<String>"</td><td>"The value of the menu item that should be active when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<String>"</td><td>"Event handler called when the value changes."</td></tr>
                    <tr><td>"orientation"</td><td>"MaybeProp<String>"</td><td>"The orientation of the menu. Default: \"horizontal\"."</td></tr>
                    <tr><td>"dir"</td><td>"MaybeProp<String>"</td><td>"The reading direction. Default: \"ltr\"."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="list">"List"</h3>
        <p>"Contains the top level menu items."</p>

        <h3 id="item">"Item"</h3>
        <p>"A top level menu item, contains a link or trigger with content combination."</p>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that toggles the content."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        <h3 id="content">"Content"</h3>
        <p>"Contains the content associated with each trigger."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
                <tr><td>"[data-motion]"</td><td>"\"to-start\" | \"to-end\" | \"from-start\" | \"from-end\""</td></tr>
            </tbody>
        </table>

        <h3 id="link">"Link"</h3>
        <p>"A navigational link."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"active"</td><td>"MaybeProp<bool>"</td><td>"Used to identify the link as the currently active page."</td></tr>
                    <tr><td>"on_select"</td><td>"Callback<ev::MouseEvent>"</td><td>"Event handler called when the user selects a link."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/examples/disclosure-navigation" style="color:var(--text-link)">"Navigation WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"When focus is on a trigger, opens the content."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"When focus is on a trigger, opens the content."</td></tr>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus to the next focusable element."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"When horizontal orientation, moves focus to the next trigger or link."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"When horizontal orientation, moves focus to the previous trigger or link."</td></tr>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Moves focus to the next trigger or link."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Moves focus to the previous trigger or link."</td></tr>
                <tr><td><kbd>"Home"</kbd></td><td>"Moves focus to the first trigger or link."</td></tr>
                <tr><td><kbd>"End"</kbd></td><td>"Moves focus to the last trigger or link."</td></tr>
                <tr><td><kbd>"Escape"</kbd></td><td>"Closes open content and moves focus to its trigger."</td></tr>
            </tbody>
        </table>
    }
}
