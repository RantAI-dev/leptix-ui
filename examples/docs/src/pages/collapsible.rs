use leptix_ui::collapsible::*;
use leptos::prelude::*;

#[component]
pub fn CollapsiblePage() -> impl IntoView {
    view! {
        <h1>"Collapsible"</h1>
        <p class="subtitle">"An interactive component which expands and collapses a content section."</p>

        <div class="features">
            <span class="feature">"Keyboard accessible"</span>
            <span class="feature">"Controlled / uncontrolled"</span>
            <span class="feature">"Disableable"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <Collapsible attr:style="width:300px">
                    <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:8px">
                        <span style="font-size:14px;font-weight:500">"Tagged items"</span>
                        <CollapsibleTrigger attr:class="demo-btn" attr:style="padding:4px 8px;font-size:12px">
                            "Toggle"
                        </CollapsibleTrigger>
                    </div>

                    <div style="padding:8px 12px;border:1px solid #e4e4e7;border-radius:6px;font-size:14px;margin-bottom:4px">
                        "@radix-ui/primitives"
                    </div>

                    <CollapsibleContent>
                        <div style="padding:8px 12px;border:1px solid #e4e4e7;border-radius:6px;font-size:14px;margin-bottom:4px">
                            "@radix-ui/colors"
                        </div>
                        <div style="padding:8px 12px;border:1px solid #e4e4e7;border-radius:6px;font-size:14px;margin-bottom:4px">
                            "@radix-ui/react-icons"
                        </div>
                    </CollapsibleContent>
                </Collapsible>
            </div>
            <div class="demo-code">{"<Collapsible>\n  <CollapsibleTrigger>\"Toggle\"</CollapsibleTrigger>\n  <div>\"Always visible item\"</div>\n  <CollapsibleContent>\n    <div>\"Hidden item 1\"</div>\n    <div>\"Hidden item 2\"</div>\n  </CollapsibleContent>\n</Collapsible>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Collapsible"</h3>
        <p>"Contains all the parts of a collapsible section."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"Controlled open state."</td></tr>
                <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"Uncontrolled initial open state."</td></tr>
                <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Called when the open state changes."</td></tr>
                <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting."</td></tr>
            </tbody>
        </table>

        <h3>"CollapsibleTrigger"</h3>
        <p>"The button that toggles the collapsible content."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Render as a child element instead of the default button."</td></tr>
            </tbody>
        </table>

        <h3>"CollapsibleContent"</h3>
        <p>"The content that is shown or hidden."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"force_mount"</td><td>"MaybeProp<bool>"</td><td>"Force mounting when more control is needed over animations."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Toggles the collapsible content when trigger is focused."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Toggles the collapsible content when trigger is focused."</td></tr>
            </tbody>
        </table>
    }
}
