use leptix_ui::avatar::*;
use leptos::prelude::*;

#[component]
pub fn AvatarPage() -> impl IntoView {
    view! {
        <h1>"Avatar"</h1>
        <p class="subtitle">"An image element with a fallback for representing the user."</p>

        <div class="features">
            <span class="feature">"Image loading"</span>
            <span class="feature">"Fallback support"</span>
            <span class="feature">"Delay fallback"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <Avatar attr:class="demo-avatar">
                    <AvatarImage src="https://i.pravatar.cc/48?u=leptix" />
                    <AvatarFallback attr:class="demo-avatar-fallback">"JD"</AvatarFallback>
                </Avatar>

                <Avatar attr:class="demo-avatar">
                    <AvatarImage src="https://broken.invalid/avatar.png" />
                    <AvatarFallback attr:class="demo-avatar-fallback">"JD"</AvatarFallback>
                </Avatar>
            </div>
            <div class="demo-code">{"<Avatar>\n  <AvatarImage src=\"https://example.com/avatar.png\" />\n  <AvatarFallback>\"JD\"</AvatarFallback>\n</Avatar>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Avatar"</h3>
        <p>"Contains all the parts of an avatar."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Render as a child element instead of the default span."</td></tr>
            </tbody>
        </table>

        <h3>"AvatarImage"</h3>
        <p>"The image to render. It will only render when loaded."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"src"</td><td>"MaybeProp<String>"</td><td>"The image source URL."</td></tr>
                <tr><td>"on_loading_status_change"</td><td>"Callback<ImageLoadingStatus>"</td><td>"Called when the loading status changes."</td></tr>
            </tbody>
        </table>

        <h3>"AvatarFallback"</h3>
        <p>"Rendered when the image has not loaded. Useful for placeholder initials or icons."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"delay_ms"</td><td>"MaybeProp<i32>"</td><td>"Delay in milliseconds before showing the fallback. Useful to prevent flashing."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"N/A"</kbd></td><td>"Avatar is a presentational component and does not require keyboard interaction."</td></tr>
            </tbody>
        </table>
    }
}
