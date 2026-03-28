use leptix_ui::progress::*;
use leptos::prelude::*;

#[component]
pub fn ProgressPage() -> impl IntoView {
    view! {
        <h1>"Progress"</h1>
        <p class="subtitle">"Displays an indicator showing the completion progress of a task."</p>

        <div class="features">
            <span class="feature">"Accessible progressbar"</span>
            <span class="feature">"Indeterminate state"</span>
            <span class="feature">"Custom value label"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <Progress value=66.0 attr:class="demo-progress">
                    <ProgressIndicator
                        attr:class="demo-progress-indicator"
                        attr:style="width: 66%"
                    />
                </Progress>
            </div>
            <div class="demo-code">{"<Progress value=66.0>\n  <ProgressIndicator\n    attr:style=\"width: 66%\"\n  />\n</Progress>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Progress"</h3>
        <p>"Contains all the parts of a progress indicator."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"value"</td><td>"MaybeProp<f64>"</td><td>"The current progress value."</td></tr>
                <tr><td>"max"</td><td>"MaybeProp<f64>"</td><td>"The maximum progress value. Default: 100."</td></tr>
                <tr><td>"get_value_label"</td><td>"Fn(f64, f64) -> String"</td><td>"A function to generate the accessible value label."</td></tr>
            </tbody>
        </table>

        <h3>"ProgressIndicator"</h3>
        <p>"Used to show the progress visually. Set its width via style to match the value."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Render as a child element instead of the default div."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"N/A"</kbd></td><td>"Progress uses role=\"progressbar\" with aria-valuenow, aria-valuemin, and aria-valuemax attributes for screen readers."</td></tr>
            </tbody>
        </table>
    }
}
