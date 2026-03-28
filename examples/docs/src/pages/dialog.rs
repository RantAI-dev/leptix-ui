use leptix_ui::dialog::*;
use leptos::prelude::*;

#[component]
pub fn DialogPage() -> impl IntoView {
    view! {
        <h1>"Dialog"</h1>
        <p class="subtitle">"A window overlaid on the primary window, rendering content in a layer above the page."</p>

        <div class="features">
            <span class="feature">"Focus trap"</span>
            <span class="feature">"Esc to close"</span>
            <span class="feature">"Click outside"</span>
            <span class="feature">"Screen reader"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <Dialog>
                    <DialogTrigger attr:class="demo-btn demo-btn-primary">
                        "Edit Profile"
                    </DialogTrigger>
                    <DialogPortal>
                        <DialogOverlay attr:class="demo-dialog-overlay" />
                        <DialogContent attr:class="demo-dialog-content">
                            <DialogTitle attr:style="font-size:18px;font-weight:600;margin-bottom:8px">
                                "Edit profile"
                            </DialogTitle>
                            <DialogDescription attr:style="color:#71717a;margin-bottom:20px;font-size:14px">
                                "Make changes to your profile here. Click save when you're done."
                            </DialogDescription>

                            <div style="display:flex;flex-direction:column;gap:12px;margin-bottom:20px">
                                <div style="display:flex;align-items:center;gap:12px">
                                    <label r#for="name" style="width:80px;text-align:right;font-size:14px">"Name"</label>
                                    <input id="name" class="demo-input" value="John Doe" style="flex:1" />
                                </div>
                                <div style="display:flex;align-items:center;gap:12px">
                                    <label r#for="username" style="width:80px;text-align:right;font-size:14px">"Username"</label>
                                    <input id="username" class="demo-input" value="@johndoe" style="flex:1" />
                                </div>
                            </div>

                            <div style="display:flex;justify-content:flex-end;gap:8px">
                                <DialogClose attr:class="demo-btn">"Cancel"</DialogClose>
                                <DialogClose attr:class="demo-btn demo-btn-primary">"Save changes"</DialogClose>
                            </div>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>
            </div>
            <div class="demo-code">{"<Dialog>\n  <DialogTrigger>\"Edit Profile\"</DialogTrigger>\n  <DialogPortal>\n    <DialogOverlay />\n    <DialogContent>\n      <DialogTitle>\"Edit profile\"</DialogTitle>\n      <DialogDescription>\"Make changes...\"</DialogDescription>\n      <DialogClose>\"Save changes\"</DialogClose>\n    </DialogContent>\n  </DialogPortal>\n</Dialog>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Dialog"</h3>
        <p>"Contains all the parts of a dialog."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"open"</td><td>"MaybeProp<bool>"</td><td>"Controlled open state."</td></tr>
                <tr><td>"default_open"</td><td>"MaybeProp<bool>"</td><td>"Uncontrolled initial open state."</td></tr>
                <tr><td>"on_open_change"</td><td>"Callback<bool>"</td><td>"Called when the open state changes."</td></tr>
                <tr><td>"modal"</td><td>"bool"</td><td>"When true, disables outside interaction. Default: true."</td></tr>
            </tbody>
        </table>

        <h3>"DialogContent"</h3>
        <p>"The content rendered when the dialog is open."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"on_open_auto_focus"</td><td>"Callback<Event>"</td><td>"Called when focus enters after opening."</td></tr>
                <tr><td>"on_close_auto_focus"</td><td>"Callback<Event>"</td><td>"Called when focus returns after closing."</td></tr>
                <tr><td>"on_escape_key_down"</td><td>"Callback<KeyboardEvent>"</td><td>"Called when Escape is pressed."</td></tr>
                <tr><td>"on_pointer_down_outside"</td><td>"Callback<PointerEvent>"</td><td>"Called when clicking outside."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"Space"</kbd></td><td>"Opens/closes the dialog when trigger is focused."</td></tr>
                <tr><td><kbd>"Enter"</kbd></td><td>"Opens/closes the dialog when trigger is focused."</td></tr>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus to next focusable element inside the dialog."</td></tr>
                <tr><td><kbd>"Shift+Tab"</kbd></td><td>"Moves focus to previous focusable element."</td></tr>
                <tr><td><kbd>"Esc"</kbd></td><td>"Closes the dialog."</td></tr>
            </tbody>
        </table>
    }
}
