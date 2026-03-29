use leptix_ui::toast::*;
use leptos::prelude::*;

#[component]
pub fn ToastPage() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <h1>"Toast"</h1>
        <p class="description">
            "A succinct message that is displayed temporarily."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="toast">
            <div class="hero-demo-card">
            <div class="demo-toast">
                <ToastProvider>
                    <button
                        on:click=move |_| set_open.set(true)
                        style="padding:0.5rem 1.25rem;border-radius:6px;background:var(--accent);color:#fff;border:none;cursor:pointer;font-weight:500;"
                    >
                        "Show Toast"
                    </button>

                    <Toast open=open on_open_change=Callback::new(move |val: bool| set_open.set(val))>
                        <ToastTitle>"Event Created"</ToastTitle>
                        <ToastDescription>"Your meeting has been scheduled."</ToastDescription>
                        <ToastClose>"Dismiss"</ToastClose>
                    </Toast>

                    <ToastViewport />
                </ToastProvider>
            </div>
            </div>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Automatically closes after a configurable duration."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Pauses closing on hover, focus, and window blur."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports hotkey to jump to the toast viewport."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-toast"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_toast::*;\n\nview! {\n    <ToastProvider>\n        // trigger\n        <Toast>\n            <ToastTitle />\n            <ToastDescription />\n            <ToastClose />\n        </Toast>\n        <ToastViewport />\n    </ToastProvider>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="provider">"Provider"</h3>
        <p>"The provider that wraps your toasts and viewport."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"duration"</td><td>"MaybeProp<i32>"</td><td>"Time in ms before the toast automatically closes. Default: 5000."</td></tr>
                    <tr><td>"swipe_direction"</td><td>"MaybeProp<SwipeDirection>"</td><td>"Direction of pointer swipe to close. Default: Right."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="toast">"Toast"</h3>
        <p>"The toast that pops up."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state."</td></tr>
                    <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the open state changes."</td></tr>
                    <tr><td>"duration"</td><td>"MaybeProp<i32>"</td><td>"Override the provider duration for this specific toast."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="viewport">"Viewport"</h3>
        <p>"A fixed region where toasts appear. Users can jump to by pressing a hotkey."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/alert" style="color:var(--text-link)">"Alert WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"F8"</kbd></td><td>"Focuses the toast viewport."</td></tr>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus to the next focusable element."</td></tr>
                <tr><td><kbd>"Escape"</kbd></td><td>"Dismisses the focused toast."</td></tr>
            </tbody>
        </table>
    }
}
