use leptix_ui::progress::*;
use leptos::prelude::*;

#[component]
pub fn ProgressPage() -> impl IntoView {
    view! {
        <h1>"Progress"</h1>
        <p class="description">
            "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar."
        </p>

        // ---- Live Demo ----
        <div class="hero-container">
            <Progress value=66.0 attr:class="demo-progress">
                <ProgressIndicator
                    attr:class="demo-progress-indicator"
                    attr:style="width: 66%"
                />
            </Progress>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Provides context for assistive technology to read the progress of a task."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports indeterminate state."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Customizable value label for screen readers."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-progress"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_progress::*;\n\nview! {\n    <Progress>\n        <ProgressIndicator />\n    </Progress>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all of the progress parts."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"MaybeProp<f64>"</td><td>"The progress value."</td></tr>
                    <tr><td>"max"</td><td>"MaybeProp<f64>"</td><td>"The maximum progress value. Default: 100."</td></tr>
                    <tr><td>"get_value_label"</td><td>"Fn(f64, f64) -> String"</td><td>"A function to get the accessible label text representing the current value in a human-readable format."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"complete\" | \"indeterminate\" | \"loading\""</td></tr>
                <tr><td>"[data-value]"</td><td>"The current value"</td></tr>
                <tr><td>"[data-max]"</td><td>"The max value"</td></tr>
            </tbody>
        </table>

        <h3 id="indicator">"Indicator"</h3>
        <p>"Used to show the progress visually. It also makes progress accessible to assistive technologies."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"as_child"</td><td>"MaybeProp<bool>"</td><td>"Merge props onto the child element instead of rendering a default div."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"complete\" | \"indeterminate\" | \"loading\""</td></tr>
                <tr><td>"[data-value]"</td><td>"The current value"</td></tr>
                <tr><td>"[data-max]"</td><td>"The max value"</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/meter" style="color:var(--text-link)">"progressbar role requirements"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"N/A"</kbd></td><td>"Progress is not interactive. It uses role=\"progressbar\" with aria-valuenow, aria-valuemin, and aria-valuemax for assistive technologies."</td></tr>
            </tbody>
        </table>
    }
}
