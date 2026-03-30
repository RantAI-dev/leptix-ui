use crate::HeroCodeBlock;
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

                    <Toast open=open on_open_change=Callback::new(move |val: bool| set_open.set(val))
                        attr:style="background:var(--bg-panel);border-radius:8px;padding:15px;box-shadow:0 10px 38px -10px rgba(22,23,24,0.35),0 10px 20px -15px rgba(22,23,24,0.2);display:grid;grid-template-areas:'title action' 'description action';grid-template-columns:auto max-content;column-gap:15px;align-items:center;animation:toastSlideIn 0.3s cubic-bezier(0.16,1,0.3,1)"
                    >
                        <ToastTitle attr:style="grid-area:title;font-weight:600;font-size:15px;margin-bottom:2px">"Event Created"</ToastTitle>
                        <ToastDescription attr:style="grid-area:description;font-size:13px;color:#888;line-height:1.4;margin:0">"Your meeting has been scheduled."</ToastDescription>
                        <ToastClose attr:style="grid-area:action;all:unset;display:inline-flex;align-items:center;justify-content:center;padding:0 12px;height:28px;border-radius:4px;font-size:12px;font-weight:500;cursor:pointer;background:var(--accent);color:#fff">"Dismiss"</ToastClose>
                    </Toast>

                    <ToastViewport attr:style="position:fixed;bottom:0;right:0;display:flex;flex-direction:column;padding:25px;gap:10px;width:390px;max-width:100vw;z-index:2147483647;outline:none;margin:0;list-style:none" />
                </ToastProvider>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_ui::toast::*;\n\nlet (open, set_open) = signal(false);\n\nview! {\n    <ToastProvider>\n        <button on:click=move |_| set_open.set(true)>\"Show Toast\"</button>\n        <Toast open=open on_open_change=Callback::new(move |val: bool| set_open.set(val))\n            attr:class=\"ToastRoot\">\n            <ToastTitle attr:class=\"ToastTitle\">\"Event Created\"</ToastTitle>\n            <ToastDescription attr:class=\"ToastDescription\">\"Your meeting has been scheduled.\"</ToastDescription>\n            <ToastClose attr:class=\"ToastClose\">\"Dismiss\"</ToastClose>\n        </Toast>\n        <ToastViewport attr:class=\"ToastViewport\" />\n    </ToastProvider>\n}"
            css_styles=".ToastViewport {\n  position: fixed;\n  bottom: 0;\n  right: 0;\n  display: flex;\n  flex-direction: column;\n  padding: 25px;\n  gap: 10px;\n  width: 390px;\n  max-width: 100vw;\n  z-index: 2147483647;\n  outline: none;\n}\n\n.ToastRoot {\n  background-color: white;\n  border-radius: 6px;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n  padding: 15px;\n  display: grid;\n  grid-template-areas: \"title action\" \"description action\";\n  grid-template-columns: auto max-content;\n  column-gap: 15px;\n  align-items: center;\n}\n\n.ToastTitle {\n  grid-area: title;\n  margin-bottom: 5px;\n  font-weight: 500;\n  color: var(--slate-12);\n  font-size: 15px;\n}\n\n.ToastDescription {\n  grid-area: description;\n  margin: 0;\n  color: var(--slate-11);\n  font-size: 13px;\n  line-height: 1.3;\n}\n\n.ToastClose {\n  grid-area: action;\n  font-size: 12px;\n}"
            modules_usage="use leptix_ui::toast::*;\n// use styles from module\n\nlet (open, set_open) = signal(false);\n\nview! {\n    <ToastProvider>\n        <button on:click=move |_| set_open.set(true)>\"Show Toast\"</button>\n        <Toast open=open on_open_change=Callback::new(move |val: bool| set_open.set(val))\n            attr:class=styles.root>\n            <ToastTitle attr:class=styles.title>\"Event Created\"</ToastTitle>\n            <ToastDescription attr:class=styles.description>\"Your meeting has been scheduled.\"</ToastDescription>\n            <ToastClose attr:class=styles.close>\"Dismiss\"</ToastClose>\n        </Toast>\n        <ToastViewport attr:class=styles.viewport />\n    </ToastProvider>\n}"
            modules_styles=".viewport {\n  position: fixed;\n  bottom: 0;\n  right: 0;\n  display: flex;\n  flex-direction: column;\n  padding: 25px;\n  gap: 10px;\n  width: 390px;\n  max-width: 100vw;\n  z-index: 2147483647;\n  outline: none;\n}\n\n.root {\n  background-color: white;\n  border-radius: 6px;\n  box-shadow: 0 10px 38px -10px rgba(22,23,24,.35),\n    0 10px 20px -15px rgba(22,23,24,.2);\n  padding: 15px;\n  display: grid;\n  grid-template-areas: \"title action\" \"description action\";\n  grid-template-columns: auto max-content;\n  column-gap: 15px;\n  align-items: center;\n}\n\n.title {\n  grid-area: title;\n  margin-bottom: 5px;\n  font-weight: 500;\n  color: var(--slate-12);\n  font-size: 15px;\n}\n\n.description {\n  grid-area: description;\n  margin: 0;\n  color: var(--slate-11);\n  font-size: 13px;\n  line-height: 1.3;\n}\n\n.close {\n  grid-area: action;\n  font-size: 12px;\n}"
            tailwind_usage="use leptix_ui::toast::*;\n\nlet (open, set_open) = signal(false);\n\nview! {\n    <ToastProvider>\n        <button on:click=move |_| set_open.set(true)\n            class=\"px-5 py-2 rounded-md bg-violet-500 text-white font-medium\">\n            \"Show Toast\"\n        </button>\n        <Toast open=open on_open_change=Callback::new(move |val: bool| set_open.set(val))\n            attr:class=\"bg-white rounded-md shadow-lg p-4 grid grid-cols-[auto_max-content] gap-x-4 items-center\">\n            <ToastTitle attr:class=\"mb-1 font-medium text-slate-900 text-sm\">\n                \"Event Created\"\n            </ToastTitle>\n            <ToastDescription attr:class=\"text-slate-500 text-xs leading-tight\">\n                \"Your meeting has been scheduled.\"\n            </ToastDescription>\n            <ToastClose attr:class=\"text-xs\">\"Dismiss\"</ToastClose>\n        </Toast>\n        <ToastViewport attr:class=\"fixed bottom-0 right-0 flex flex-col p-6 gap-2.5 w-[390px] max-w-screen z-[2147483647] outline-none\" />\n    </ToastProvider>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      width: {\n        toast: \"390px\",\n      },\n      gridTemplateColumns: {\n        toast: \"auto max-content\",\n      },\n    },\n  },\n} satisfies Config;"
        />

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
