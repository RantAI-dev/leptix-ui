use leptix_ui::tabs::*;
use leptos::prelude::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    view! {
        <h1>"Tabs"</h1>
        <p class="description">
            "A set of layered sections of content\u{2014}known as tab panels\u{2014}that are displayed one at a time."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="tabs">
            <div class="hero-demo-card">
            <Tabs default_value="account" attr:style="width:100%;max-width:400px">
                <TabsList attr:class="demo-tabs-list">
                    <TabsTrigger value="account" attr:class="demo-tabs-trigger">"Account"</TabsTrigger>
                    <TabsTrigger value="password" attr:class="demo-tabs-trigger">"Password"</TabsTrigger>
                    <TabsTrigger value="settings" attr:class="demo-tabs-trigger">"Settings"</TabsTrigger>
                </TabsList>
                <TabsContent value="account" attr:class="demo-tabs-content">
                    <p style="margin:0;font-size:14px;color:var(--text-secondary)">"Make changes to your account here. Update your name, email, and profile picture."</p>
                </TabsContent>
                <TabsContent value="password" attr:class="demo-tabs-content">
                    <p style="margin:0;font-size:14px;color:var(--text-secondary)">"Change your password here. After saving, you will be logged out."</p>
                </TabsContent>
                <TabsContent value="settings" attr:class="demo-tabs-content">
                    <p style="margin:0;font-size:14px;color:var(--text-secondary)">"Manage your notification preferences and application settings."</p>
                </TabsContent>
            </Tabs>
            </div>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports horizontal and vertical orientation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports automatic or manual activation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-tabs"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_tabs::*;\n\nview! {\n    <Tabs default_value=\"account\">\n        <TabsList>\n            <TabsTrigger value=\"account\" />\n            <TabsTrigger value=\"password\" />\n        </TabsList>\n        <TabsContent value=\"account\" />\n        <TabsContent value=\"password\" />\n    </Tabs>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the tabs component parts."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The controlled value of the tab to activate."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<String>"</td><td>"The value of the tab that should be active when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<String>"</td><td>"Event handler called when the value changes."</td></tr>
                    <tr><td>"orientation"</td><td>"MaybeProp<String>"</td><td>"The orientation of the component. Default: \"horizontal\"."</td></tr>
                    <tr><td>"activation_mode"</td><td>"ActivationMode"</td><td>"Whether tabs are activated automatically on focus or manually on click. Default: Automatic."</td></tr>
                    <tr><td>"dir"</td><td>"MaybeProp<Direction>"</td><td>"The reading direction of the tabs. If omitted, inherits globally or assumes LTR."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="list">"List"</h3>
        <p>"Contains the triggers that are aligned along the edge of the active content."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that activates its associated content."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"String"</td><td>"A unique value that associates the trigger with a content."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the tab."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"active\" | \"inactive\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="content">"Content"</h3>
        <p>"Contains the content associated with each trigger."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"String"</td><td>"A unique value that associates the content with a trigger."</td></tr>
                    <tr><td>"force_mount"</td><td>"MaybeProp<bool>"</td><td>"Used to force mounting when more control is needed. Useful when controlling animation with animation libraries."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"active\" | \"inactive\""</td></tr>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/tabs" style="color:var(--text-link)">"Tabs WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Tab"</kbd></td><td>"When focus moves onto the tabs, focuses the active trigger. When a trigger is focused, moves focus to the active content."</td></tr>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Moves focus to the next trigger (horizontal orientation)."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Moves focus to the previous trigger (horizontal orientation)."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Moves focus to the next trigger (vertical orientation)."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Moves focus to the previous trigger (vertical orientation)."</td></tr>
                <tr><td><kbd>"Home"</kbd></td><td>"Moves focus to the first trigger."</td></tr>
                <tr><td><kbd>"End"</kbd></td><td>"Moves focus to the last trigger."</td></tr>
            </tbody>
        </table>
    }
}
