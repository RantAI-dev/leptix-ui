use leptix_ui::alert_dialog::*;
use leptos::prelude::*;

#[component]
pub fn AlertDialogPage() -> impl IntoView {
    view! {
        <h1>"Alert Dialog"</h1>
        <p class="subtitle">"A modal dialog that interrupts the user with important content and expects a response. Cannot be dismissed by clicking outside."</p>

        <div class="features">
            <span class="feature">"Focus trap"</span>
            <span class="feature">"Esc to close"</span>
            <span class="feature">"No outside dismiss"</span>
            <span class="feature">"Screen reader"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <AlertDialog>
                    <AlertDialogTrigger attr:class="demo-btn demo-btn-primary">
                        "Delete Account"
                    </AlertDialogTrigger>
                    <DialogPortal>
                        <AlertDialogOverlay attr:class="demo-dialog-overlay" />
                        <AlertDialogContent attr:class="demo-dialog-content">
                            <AlertDialogTitle attr:style="font-size:18px;font-weight:600;margin-bottom:8px">
                                "Are you absolutely sure?"
                            </AlertDialogTitle>
                            <AlertDialogDescription attr:style="color:#71717a;margin-bottom:20px;font-size:14px">
                                "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                            </AlertDialogDescription>

                            <div style="display:flex;justify-content:flex-end;gap:8px">
                                <AlertDialogCancel attr:class="demo-btn">"Cancel"</AlertDialogCancel>
                                <AlertDialogAction attr:class="demo-btn demo-btn-primary">"Yes, delete account"</AlertDialogAction>
                            </div>
                        </AlertDialogContent>
                    </DialogPortal>
                </AlertDialog>
            </div>
            <div class="demo-code">{"<AlertDialog>\n  <AlertDialogTrigger>\"Delete Account\"</AlertDialogTrigger>\n  <DialogPortal>\n    <AlertDialogOverlay />\n    <AlertDialogContent>\n      <AlertDialogTitle>\"Are you sure?\"</AlertDialogTitle>\n      <AlertDialogDescription>\"This action cannot be undone.\"</AlertDialogDescription>\n      <AlertDialogCancel>\"Cancel\"</AlertDialogCancel>\n      <AlertDialogAction>\"Yes, delete\"</AlertDialogAction>\n    </AlertDialogContent>\n  </DialogPortal>\n</AlertDialog>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"AlertDialog"</h3>
        <p>"Contains all the parts of an alert dialog."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"Controlled open state."</td></tr>
                <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"Uncontrolled initial open state."</td></tr>
                <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Called when the open state changes."</td></tr>
            </tbody>
        </table>

        <h3>"AlertDialogContent"</h3>
        <p>"The content rendered when the alert dialog is open. Clicking outside will not dismiss it."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"on_open_auto_focus"</td><td>"Callback<Event>"</td><td>"Called when focus enters after opening."</td></tr>
                <tr><td>"on_close_auto_focus"</td><td>"Callback<Event>"</td><td>"Called when focus returns after closing."</td></tr>
                <tr><td>"on_escape_key_down"</td><td>"Callback<KeyboardEvent>"</td><td>"Called when Escape is pressed."</td></tr>
            </tbody>
        </table>

        <h3>"AlertDialogCancel"</h3>
        <p>"A button that closes the alert dialog, indicating cancellation."</p>

        <h3>"AlertDialogAction"</h3>
        <p>"A button that closes the alert dialog, indicating confirmation of the action."</p>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Opens/closes the alert dialog when trigger is focused."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Opens/closes the alert dialog when trigger is focused."</td></tr>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus to next focusable element inside the dialog."</td></tr>
                <tr><td><kbd>"Shift+Tab"</kbd></td><td>"Moves focus to previous focusable element."</td></tr>
                <tr><td><kbd>"Esc"</kbd></td><td>"Closes the alert dialog."</td></tr>
            </tbody>
        </table>
    }
}
