use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Leptix UI"</h1>
        <p class="subtitle">"Radix-style accessible UI primitives for Leptos."</p>

        <div class="features">
            <span class="feature">"Accessible"</span>
            <span class="feature">"Unstyled"</span>
            <span class="feature">"Composable"</span>
            <span class="feature">"31 Components"</span>
        </div>

        <p>"Leptix provides a set of low-level, accessible UI components for building design systems and web applications with Leptos. Each component follows WAI-ARIA patterns and handles keyboard navigation, focus management, and screen reader support."</p>

        <h2>"Installation"</h2>
        <div class="install-code">"cargo add leptix-ui"</div>

        <p>"Or pick individual components:"</p>
        <div class="install-code">"cargo add leptix-dialog leptix-tabs leptix-switch"</div>

        <h2>"Principles"</h2>

        <h3>"Unstyled"</h3>
        <p>"Components ship with zero CSS. You bring your own styles using data-state selectors, classes, or Tailwind."</p>

        <h3>"Accessible"</h3>
        <p>"Every component implements WAI-ARIA design patterns with correct roles, keyboard interactions, and screen reader announcements."</p>

        <h3>"Composable"</h3>
        <p>"Each component is built from small, composable parts. Use only the pieces you need."</p>

        <h2>"Browse Components"</h2>
        <p>"Use the sidebar to explore each component with live interactive demos."</p>
    }
}
