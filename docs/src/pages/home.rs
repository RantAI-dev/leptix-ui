use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        // ---- Hero ----
        <div class="home-hero">
            <img src="/public/logo/logo.png" alt="Leptix" width="80" height="80" style="margin-bottom:16px" />
            <h1 style="font-size:52px;font-weight:800;letter-spacing:-0.04em;line-height:1.05;margin:0">"Radix Primitives"<br/>"for Leptos"</h1>
            <p style="font-size:19px;color:var(--text-secondary);max-width:520px;margin:16px auto 0;line-height:1.6">
                "An open-source Rust component library of 33 unstyled, accessible primitives for building high-quality design systems and web apps."
            </p>

            <div style="display:flex;gap:12px;margin-top:28px;justify-content:center;flex-wrap:wrap">
                <a href="/dialog" class="home-btn home-btn-primary">"Browse Components"</a>
                <a href="https://github.com/RantAI-dev/leptix-ui" target="_blank" class="home-btn home-btn-outline">"GitHub"</a>
                <a href="https://crates.io/crates/leptix-ui" target="_blank" class="home-btn home-btn-outline">"crates.io"</a>
            </div>
        </div>

        // ---- Feature Grid ----
        <div class="home-features">
            <div class="home-feature-card">
                <div class="home-feature-icon" style="background:#e8f5e9;color:#2e7d32">"A"</div>
                <h3>"Accessible"</h3>
                <p>"WAI-ARIA compliant. Keyboard navigation, focus management, and screen reader support are built in — not bolted on."</p>
            </div>
            <div class="home-feature-card">
                <div class="home-feature-icon" style="background:#e3f2fd;color:#1565c0">"U"</div>
                <h3>"Unstyled"</h3>
                <p>"Ships with zero CSS. Style with your design system using data-state selectors, classes, or Tailwind."</p>
            </div>
            <div class="home-feature-card">
                <div class="home-feature-icon" style="background:#fce4ec;color:#c62828">"C"</div>
                <h3>"Composable"</h3>
                <p>"Each component is built from small, composable parts. Use only the pieces you need."</p>
            </div>
            <div class="home-feature-card">
                <div class="home-feature-icon" style="background:#fff3e0;color:#e65100">"F"</div>
                <h3>"Full-Featured"</h3>
                <p>"Floating UI positioning, portal rendering, swipe gestures, typeahead search, RTL support — all out of the box."</p>
            </div>
        </div>

        // ---- Install ----
        <div class="home-install">
            <h2 style="text-align:center;margin-bottom:8px">"Get Started"</h2>
            <p style="text-align:center;color:var(--text-secondary);margin-bottom:20px">"Install the umbrella crate or pick individual components."</p>
            <div class="install-block" style="max-width:500px;margin:0 auto">
                <span><span class="prompt">"$"</span>" cargo add leptix-ui"</span>
            </div>
            <p style="text-align:center;color:var(--text-secondary);font-size:14px;margin-top:12px">"Or cherry-pick:"</p>
            <div class="install-block" style="max-width:500px;margin:0 auto">
                <span><span class="prompt">"$"</span>" cargo add leptix-dialog leptix-tabs leptix-tooltip"</span>
            </div>
        </div>

        // ---- Components Count ----
        <div style="text-align:center;padding:40px 0 20px">
            <p style="font-size:16px;color:var(--text-secondary);margin:0">
                "33 components covering forms, overlays, navigation, layout, and more."<br/>
                "Click any component in the sidebar to see a live interactive demo."
            </p>
        </div>

        // ---- Built by ----
        <div style="text-align:center;padding:20px 0 40px;border-top:1px solid var(--border);margin-top:20px">
            <p style="font-size:14px;color:var(--text-secondary);margin:0">
                "Built by "
                <a href="https://github.com/RantAI-dev" target="_blank" style="color:var(--accent);font-weight:600;text-decoration:none">"RantAI"</a>
                " \u{2022} "
                <a href="https://github.com/RantAI-dev/leptix-ui" target="_blank" style="color:var(--text-secondary);text-decoration:none">"MIT Licensed"</a>
            </p>
        </div>
    }
}
