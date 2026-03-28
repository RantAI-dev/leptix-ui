use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Primitives"</h1>
        <p class="description">"Unstyled, accessible, open source Leptos components for building high-quality design systems and web apps."</p>

        <div class="hero-container" style="flex-direction:column;gap:16px;text-align:center;padding:60px 40px">
            <div style="font-size:48px;font-weight:700;letter-spacing:-0.03em;line-height:1.1">"Build your"<br/>"component library"</div>
            <p style="font-size:18px;color:var(--text-secondary);max-width:500px;margin:0 auto">"Open source Leptos components that are unstyled, accessible, and built for composition."</p>

            <div style="display:flex;gap:12px;margin-top:8px">
                <a href="/dialog" class="demo-btn demo-btn-accent" style="text-decoration:none">"Browse Components"</a>
            </div>
        </div>

        <h2>"Features"</h2>

        <div style="display:grid;grid-template-columns:1fr 1fr;gap:24px;margin-bottom:32px">
            <div>
                <h3 style="margin-top:0">"Accessible"</h3>
                <p>"Components adhere to WAI-ARIA design patterns. Keyboard navigation, focus management, and screen reader support are built in."</p>
            </div>
            <div>
                <h3 style="margin-top:0">"Unstyled"</h3>
                <p>"Ships with zero CSS. Style with your own design system using data-state selectors, classes, or Tailwind CSS."</p>
            </div>
            <div>
                <h3 style="margin-top:0">"Composable"</h3>
                <p>"Each component is built from small, composable parts. Use only the pieces you need."</p>
            </div>
            <div>
                <h3 style="margin-top:0">"Uncontrolled or Controlled"</h3>
                <p>"Every stateful component can be used as an uncontrolled component with sensible defaults, or controlled with full state management."</p>
            </div>
        </div>

        <h2>"Installation"</h2>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-ui"</span>
        </div>
        <p>"Or pick individual components:"</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-dialog leptix-tabs leptix-switch"</span>
        </div>

        <h2>"Components"</h2>
        <p>"31 components covering forms, overlays, navigation, layout, and more. Click any component in the sidebar to see a live interactive demo."</p>
    }
}
