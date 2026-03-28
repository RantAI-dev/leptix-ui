use leptix_ui::separator::*;
use leptos::prelude::*;

#[component]
pub fn SeparatorPage() -> impl IntoView {
    view! {
        <h1>"Separator"</h1>
        <p class="subtitle">"Visually or semantically separates content."</p>

        <div class="features">
            <span class="feature">"Horizontal / vertical"</span>
            <span class="feature">"Decorative mode"</span>
            <span class="feature">"Accessible"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <div style="width:300px">
                    <div style="font-size:15px;font-weight:600">"Leptix Primitives"</div>
                    <div style="font-size:14px;color:#71717a;margin-top:4px">"An accessible component library for Leptos."</div>
                    <Separator attr:class="demo-separator" attr:style="margin:16px 0" />
                    <div style="display:flex;align-items:center;gap:12px;font-size:14px">
                        <span>"Blog"</span>
                        <Separator orientation=Orientation::Vertical attr:class="demo-separator" />
                        <span>"Docs"</span>
                        <Separator orientation=Orientation::Vertical attr:class="demo-separator" />
                        <span>"Source"</span>
                    </div>
                </div>
            </div>
            <div class="demo-code">{"<Separator />\n\n<Separator orientation=Orientation::Vertical />"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Separator"</h3>
        <p>"The separator element."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"orientation"</td><td>"MaybeProp<Orientation>"</td><td>"The orientation of the separator. Default: Horizontal."</td></tr>
                <tr><td>"decorative"</td><td>"MaybeProp<bool>"</td><td>"When true, uses role=\"none\" instead of role=\"separator\". Default: false."</td></tr>
                <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Render as a child element instead of the default div."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"N/A"</kbd></td><td>"Separator is not interactive. It uses role=\"separator\" with aria-orientation for screen readers, or role=\"none\" when decorative."</td></tr>
            </tbody>
        </table>
    }
}
