use leptix_ui::label::*;
use leptos::prelude::*;

#[component]
pub fn LabelPage() -> impl IntoView {
    view! {
        <h1>"Label"</h1>
        <p class="subtitle">"Renders an accessible label associated with controls. Prevents text selection on double-click."</p>

        <div class="features">
            <span class="feature">"Native label element"</span>
            <span class="feature">"Double-click safe"</span>
            <span class="feature">"Accessible"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <div style="display:flex;align-items:center;gap:12px">
                    <Label attr:class="demo-label" attr:r#for="demo-email">
                        "Email address"
                    </Label>
                    <input id="demo-email" class="demo-input" type="email" placeholder="you@example.com" />
                </div>
            </div>
            <div class="demo-code">{"<Label r#for=\"email\">\"Email address\"</Label>\n<input id=\"email\" type=\"email\" />"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Label"</h3>
        <p>"The label element. Renders a native <label>."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"on_mouse_down"</td><td>"MaybeCallback<MouseEvent>"</td><td>"Called on mouse down. Prevents text selection on double-click by default."</td></tr>
                <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Render as a child element instead of the default label."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"N/A"</kbd></td><td>"Label is a native element. Clicking the label focuses the associated control via the for attribute."</td></tr>
            </tbody>
        </table>
    }
}
