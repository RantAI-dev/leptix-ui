use crate::HeroCodeBlock;
use leptix_ui::dialog::*;
use leptos::prelude::*;

#[component]
pub fn DialogPage() -> impl IntoView {
    view! {
        <h1>"Dialog"</h1>
        <p class="description">
            "A window overlaid on either the primary window or another dialog window, rendering the content underneath inert."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="dialog">
            <div class="hero-demo-card" style="padding:24px 32px">
                <p style="color:var(--text-secondary);font-size:14px;margin:0 0 12px;text-align:center">"Click the button to open the dialog"</p>
                <div style="text-align:center">
            <Dialog>
                <DialogTrigger attr:class="demo-btn demo-btn-accent">
                    "Edit Profile"
                </DialogTrigger>
                <DialogPortal>
                    <DialogOverlay attr:class="demo-dialog-overlay" />
                    <DialogContent attr:class="demo-dialog-content">
                        <DialogTitle attr:style="font-size:17px;font-weight:600;margin-bottom:4px">
                            "Edit profile"
                        </DialogTitle>
                        <DialogDescription attr:style="color:var(--text-secondary);margin-bottom:20px;font-size:14px">
                            "Make changes to your profile here. Click save when you\u{2019}re done."
                        </DialogDescription>

                        <div style="display:flex;flex-direction:column;gap:10px;margin-bottom:20px">
                            <div style="display:flex;align-items:center;gap:12px">
                                <label r#for="name" style="width:80px;text-align:right;font-size:14px;color:var(--text-secondary)">"Name"</label>
                                <input id="name" class="demo-input" value="Pedro Duarte" style="flex:1" />
                            </div>
                            <div style="display:flex;align-items:center;gap:12px">
                                <label r#for="username" style="width:80px;text-align:right;font-size:14px;color:var(--text-secondary)">"Username"</label>
                                <input id="username" class="demo-input" value="@peduarte" style="flex:1" />
                            </div>
                        </div>

                        <div style="display:flex;justify-content:flex-end">
                            <DialogClose attr:class="demo-btn demo-btn-accent">"Save changes"</DialogClose>
                        </div>

                        <DialogClose attr:style="position:absolute;top:12px;right:12px;border:none;background:none;cursor:pointer;font-size:15px;color:var(--text-muted);padding:4px">"✕"</DialogClose>
                    </DialogContent>
                </DialogPortal>
            </Dialog>
                </div>
            </div>
        </div>
        <HeroCodeBlock
            css_usage="use leptix_ui::dialog::*;\n\nview! {\n    <Dialog>\n        <DialogTrigger>\"Edit Profile\"</DialogTrigger>\n        <DialogPortal>\n            <DialogOverlay attr:class=\"DialogOverlay\" />\n            <DialogContent attr:class=\"DialogContent\">\n                <DialogTitle>\"Edit profile\"</DialogTitle>\n                <DialogDescription>\"Make changes to your profile here.\"</DialogDescription>\n                <DialogClose>\"Save changes\"</DialogClose>\n            </DialogContent>\n        </DialogPortal>\n    </Dialog>\n}"
            css_styles=".DialogOverlay {\n  background-color: var(--black-a9);\n  position: fixed;\n  inset: 0;\n  animation: overlayShow 150ms cubic-bezier(0.16, 1, 0.3, 1);\n}\n\n.DialogContent {\n  background-color: white;\n  border-radius: 6px;\n  box-shadow: hsl(206 22% 7% / 35%) 0px 10px 38px -10px,\n    hsl(206 22% 7% / 20%) 0px 10px 20px -15px;\n  position: fixed;\n  top: 50%;\n  left: 50%;\n  transform: translate(-50%, -50%);\n  width: 90vw;\n  max-width: 450px;\n  max-height: 85vh;\n  padding: 25px;\n  animation: contentShow 150ms cubic-bezier(0.16, 1, 0.3, 1);\n}\n\n@keyframes overlayShow {\n  from { opacity: 0; }\n  to { opacity: 1; }\n}\n@keyframes contentShow {\n  from { opacity: 0; transform: translate(-50%, -48%) scale(0.96); }\n  to { opacity: 1; transform: translate(-50%, -50%) scale(1); }\n}"
            modules_usage="// import styles from \"./dialog.module.css\";\nuse leptix_ui::dialog::*;\n\nview! {\n    <Dialog>\n        <DialogTrigger>\"Edit Profile\"</DialogTrigger>\n        <DialogPortal>\n            <DialogOverlay attr:class=styles.overlay />\n            <DialogContent attr:class=styles.content>\n                <DialogTitle>\"Edit profile\"</DialogTitle>\n                <DialogDescription>\"Make changes to your profile here.\"</DialogDescription>\n                <DialogClose>\"Save changes\"</DialogClose>\n            </DialogContent>\n        </DialogPortal>\n    </Dialog>\n}"
            modules_styles=".overlay {\n  background-color: var(--black-a9);\n  position: fixed;\n  inset: 0;\n  animation: overlayShow 150ms cubic-bezier(0.16, 1, 0.3, 1);\n}\n\n.content {\n  background-color: white;\n  border-radius: 6px;\n  box-shadow: hsl(206 22% 7% / 35%) 0px 10px 38px -10px,\n    hsl(206 22% 7% / 20%) 0px 10px 20px -15px;\n  position: fixed;\n  top: 50%;\n  left: 50%;\n  transform: translate(-50%, -50%);\n  width: 90vw;\n  max-width: 450px;\n  max-height: 85vh;\n  padding: 25px;\n  animation: contentShow 150ms cubic-bezier(0.16, 1, 0.3, 1);\n}\n\n@keyframes overlayShow {\n  from { opacity: 0; }\n  to { opacity: 1; }\n}\n@keyframes contentShow {\n  from { opacity: 0; transform: translate(-50%, -48%) scale(0.96); }\n  to { opacity: 1; transform: translate(-50%, -50%) scale(1); }\n}"
            tailwind_usage="use leptix_ui::dialog::*;\n\nview! {\n    <Dialog>\n        <DialogTrigger>\"Edit Profile\"</DialogTrigger>\n        <DialogPortal>\n            <DialogOverlay attr:class=\"fixed inset-0 bg-black/60 animate-overlayShow\" />\n            <DialogContent attr:class=\"fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded-md shadow-xl w-[90vw] max-w-[450px] max-h-[85vh] p-6 animate-contentShow\">\n                <DialogTitle>\"Edit profile\"</DialogTitle>\n                <DialogDescription>\"Make changes to your profile here.\"</DialogDescription>\n                <DialogClose>\"Save changes\"</DialogClose>\n            </DialogContent>\n        </DialogPortal>\n    </Dialog>\n}"
            tailwind_config="import type { Config } from \"tailwindcss\";\n\nexport default {\n  theme: {\n    extend: {\n      keyframes: {\n        overlayShow: {\n          from: { opacity: \"0\" },\n          to: { opacity: \"1\" },\n        },\n        contentShow: {\n          from: { opacity: \"0\", transform: \"translate(-50%, -48%) scale(0.96)\" },\n          to: { opacity: \"1\", transform: \"translate(-50%, -50%) scale(1)\" },\n        },\n      },\n      animation: {\n        overlayShow: \"overlayShow 150ms cubic-bezier(0.16, 1, 0.3, 1)\",\n        contentShow: \"contentShow 150ms cubic-bezier(0.16, 1, 0.3, 1)\",\n      },\n    },\n  },\n} satisfies Config;"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports modal and non-modal modes."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Focus is automatically trapped within modal."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span><span>"Manages screen reader announcements with "<code>"Title"</code>" and "<code>"Description"</code>" components."</span></div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Esc closes the component automatically."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-dialog"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_dialog::*;\n\nview! {\n    <Dialog>\n        <DialogTrigger />\n        <DialogPortal>\n            <DialogOverlay />\n            <DialogContent>\n                <DialogTitle />\n                <DialogDescription />\n                <DialogClose />\n            </DialogContent>\n        </DialogPortal>\n    </Dialog>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a dialog."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state of the dialog."</td></tr>
                    <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"The open state when initially rendered. Use when you do not need to control its open state."</td></tr>
                    <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the open state changes."</td></tr>
                    <tr><td>"modal"</td><td>"bool"</td><td>"When true, interaction with outside elements is disabled and only dialog content is visible to screen readers. Default: true."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="trigger">"Trigger"</h3>
        <p>"The button that opens the dialog."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto child element instead of rendering a default button."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
            </tbody>
        </table>

        <h3 id="portal">"Portal"</h3>
        <p>"When used, portals your overlay and content parts into the body."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"container"</td><td>"MaybeProp<SendWrapper<Element>>"</td><td>"Specify a container element to portal into. Defaults to document.body."</td></tr>
                    <tr><td>"force_mount"</td><td>"MaybeProp<bool>"</td><td>"Used to force mounting when more control is needed."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="overlay">"Overlay"</h3>
        <p>"A layer that covers the inert portion of the view when the dialog is open."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
            </tbody>
        </table>

        <h3 id="content">"Content"</h3>
        <p>"Contains content to be rendered in the open dialog."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"on_open_auto_focus"</td><td>"Callback<Event>"</td><td>"Event handler called when focus moves into the component after opening. Can be prevented."</td></tr>
                    <tr><td>"on_close_auto_focus"</td><td>"Callback<Event>"</td><td>"Event handler called when focus moves to the trigger after closing. Can be prevented."</td></tr>
                    <tr><td>"on_escape_key_down"</td><td>"Callback<KeyboardEvent>"</td><td>"Event handler called when the escape key is down. Can be prevented."</td></tr>
                    <tr><td>"on_pointer_down_outside"</td><td>"Callback<PointerEvent>"</td><td>"Event handler when a pointer event happens outside the component. Can be prevented."</td></tr>
                    <tr><td>"on_interact_outside"</td><td>"Callback<Event>"</td><td>"Event handler when an interaction happens outside. Can be prevented."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
            </tbody>
        </table>

        <h3 id="close">"Close"</h3>
        <p>"The button that closes the dialog."</p>

        <h3 id="title">"Title"</h3>
        <p>"An accessible title to be announced when the dialog is opened."</p>

        <h3 id="description">"Description"</h3>
        <p>"An optional accessible description to be announced when the dialog is opened."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal" style="color:var(--text-link)">"Dialog WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Opens/closes the dialog."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Opens/closes the dialog."</td></tr>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus to the next focusable element."</td></tr>
                <tr><td><kbd>"Shift+Tab"</kbd></td><td>"Moves focus to the previous focusable element."</td></tr>
                <tr><td><kbd>"Esc"</kbd></td><td>"Closes the dialog and moves focus to the trigger."</td></tr>
            </tbody>
        </table>
    }
}
