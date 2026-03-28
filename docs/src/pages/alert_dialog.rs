use leptix_ui::alert_dialog::*;
use leptix_ui::dialog::DialogPortal;
use leptos::prelude::*;

#[component]
pub fn AlertDialogPage() -> impl IntoView {
    view! {
        <h1>"Alert Dialog"</h1>
        <p class="description">
            "A modal dialog that interrupts the user with important content and expects a response."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="alert-dialog">
            <div class="hero-demo-card">
            <AlertDialog>
                <AlertDialogTrigger attr:class="demo-btn demo-btn-accent">
                    "Delete Account"
                </AlertDialogTrigger>
                <DialogPortal>
                    <AlertDialogOverlay attr:class="demo-dialog-overlay" />
                    <AlertDialogContent attr:class="demo-dialog-content">
                        <AlertDialogTitle attr:style="font-size:17px;font-weight:600;margin-bottom:4px">
                            "Are you absolutely sure?"
                        </AlertDialogTitle>
                        <AlertDialogDescription attr:style="color:var(--text-secondary);margin-bottom:20px;font-size:14px">
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        </AlertDialogDescription>

                        <div style="display:flex;justify-content:flex-end;gap:8px">
                            <AlertDialogCancel attr:class="demo-btn">"Cancel"</AlertDialogCancel>
                            <AlertDialogAction attr:class="demo-btn demo-btn-accent">"Yes, delete account"</AlertDialogAction>
                        </div>
                    </AlertDialogContent>
                </DialogPortal>
            </AlertDialog>
            </div>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Focus is automatically trapped when open."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span><span>"Manages screen reader announcements with "<code>"Title"</code>" and "<code>"Description"</code>" components."</span></div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Esc closes the component automatically."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Clicking outside will not dismiss it."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-alert-dialog"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_alert_dialog::*;\n\nview! {\n    <AlertDialog>\n        <AlertDialogTrigger />\n        <DialogPortal>\n            <AlertDialogOverlay />\n            <AlertDialogContent>\n                <AlertDialogTitle />\n                <AlertDialogDescription />\n                <AlertDialogCancel />\n                <AlertDialogAction />\n            </AlertDialogContent>\n        </DialogPortal>\n    </AlertDialog>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of an alert dialog."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"The controlled open state of the dialog."</td></tr>
                    <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"The open state when initially rendered. Use when you do not need to control its open state."</td></tr>
                    <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Event handler called when the open state of the dialog changes."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="trigger">"Trigger"</h3>
        <p>"A button that opens the dialog."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
            </tbody>
        </table>

        <h3 id="portal">"Portal"</h3>
        <p>"When used, portals your overlay and content parts into the body."</p>

        <h3 id="overlay">"Overlay"</h3>
        <p>"A layer that covers the inert portion of the view when the dialog is open."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
            </tbody>
        </table>

        <h3 id="content">"Content"</h3>
        <p>"Contains the content to be rendered when the dialog is open. Clicking outside will not close it."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"on_open_auto_focus"</td><td>"Callback<Event>"</td><td>"Event handler called when focus moves into the component after opening. Can be prevented."</td></tr>
                    <tr><td>"on_close_auto_focus"</td><td>"Callback<Event>"</td><td>"Event handler called when focus moves to the trigger after closing. Can be prevented."</td></tr>
                    <tr><td>"on_escape_key_down"</td><td>"Callback<KeyboardEvent>"</td><td>"Event handler called when the escape key is down. Can be prevented."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"open\" | \"closed\""</td></tr>
            </tbody>
        </table>

        <h3 id="cancel">"Cancel"</h3>
        <p>"A button that closes the dialog. This button should be distinguished visually from AlertDialogAction."</p>

        <h3 id="action">"Action"</h3>
        <p>"A button that closes the dialog. These buttons should be distinguished visually from the Cancel button."</p>

        <h3 id="title">"Title"</h3>
        <p>"An accessible name to be announced when the dialog is opened."</p>

        <h3 id="description">"Description"</h3>
        <p>"An accessible description to be announced when the dialog is opened."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/alertdialog" style="color:var(--text-link)">"Alert Dialog WAI-ARIA design pattern"</a>"."</p>

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
