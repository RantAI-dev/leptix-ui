use leptix_ui::tabs::*;
use leptos::prelude::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    view! {
        <h1>"Tabs"</h1>
        <p class="subtitle">"A set of layered sections of content, known as tab panels, displayed one at a time."</p>

        <div class="features">
            <span class="feature">"Arrow key navigation"</span>
            <span class="feature">"Automatic activation"</span>
            <span class="feature">"Horizontal / vertical"</span>
            <span class="feature">"RTL support"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview" style="display:block;padding:24px 40px">
                <Tabs default_value="account" attr:style="width:100%">
                    <TabsList attr:class="demo-tabs-list">
                        <TabsTrigger value="account" attr:class="demo-tabs-trigger">"Account"</TabsTrigger>
                        <TabsTrigger value="password" attr:class="demo-tabs-trigger">"Password"</TabsTrigger>
                        <TabsTrigger value="settings" attr:class="demo-tabs-trigger">"Settings"</TabsTrigger>
                    </TabsList>
                    <TabsContent value="account" attr:class="demo-tabs-content">
                        <p style="margin:0;font-size:14px;color:#71717a">"Make changes to your account here. Update your name, email, and profile picture."</p>
                    </TabsContent>
                    <TabsContent value="password" attr:class="demo-tabs-content">
                        <p style="margin:0;font-size:14px;color:#71717a">"Change your password here. After saving, you will be logged out."</p>
                    </TabsContent>
                    <TabsContent value="settings" attr:class="demo-tabs-content">
                        <p style="margin:0;font-size:14px;color:#71717a">"Manage your notification preferences and application settings."</p>
                    </TabsContent>
                </Tabs>
            </div>
            <div class="demo-code">{"<Tabs default_value=\"account\">\n  <TabsList>\n    <TabsTrigger value=\"account\">\"Account\"</TabsTrigger>\n    <TabsTrigger value=\"password\">\"Password\"</TabsTrigger>\n  </TabsList>\n  <TabsContent value=\"account\">...</TabsContent>\n  <TabsContent value=\"password\">...</TabsContent>\n</Tabs>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Tabs"</h3>
        <p>"Contains all the tabs component parts."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"Controlled active tab value."</td></tr>
                <tr><td>"default_value"</td><td>"MaybeProp<String>"</td><td>"Uncontrolled initial active tab value."</td></tr>
                <tr><td>"on_value_change"</td><td>"Callback<String>"</td><td>"Called when the active tab changes."</td></tr>
                <tr><td>"orientation"</td><td>"MaybeProp<String>"</td><td>"The orientation of the tabs. Default: \"horizontal\"."</td></tr>
                <tr><td>"activation_mode"</td><td>"ActivationMode"</td><td>"Automatic (on focus) or Manual (on click). Default: Automatic."</td></tr>
                <tr><td>"dir"</td><td>"MaybeProp<Direction>"</td><td>"Reading direction for RTL support."</td></tr>
            </tbody>
        </table>

        <h3>"TabsTrigger"</h3>
        <p>"The button that activates its associated content."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"value"</td><td>"String"</td><td>"A unique value that associates the trigger with content."</td></tr>
                <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents user interaction."</td></tr>
            </tbody>
        </table>

        <h3>"TabsContent"</h3>
        <p>"Contains the content associated with each trigger."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"value"</td><td>"String"</td><td>"A unique value that associates the content with a trigger."</td></tr>
                <tr><td>"force_mount"</td><td>"MaybeProp<bool>"</td><td>"When true, content is always rendered in the DOM."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus into the active trigger, then into the active content."</td></tr>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Moves focus to the next trigger (horizontal)."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Moves focus to the previous trigger (horizontal)."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Moves focus to the next trigger (vertical)."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Moves focus to the previous trigger (vertical)."</td></tr>
                <tr><td><kbd>"Home"</kbd></td><td>"Moves focus to the first trigger."</td></tr>
                <tr><td><kbd>"End"</kbd></td><td>"Moves focus to the last trigger."</td></tr>
            </tbody>
        </table>
    }
}
