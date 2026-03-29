use leptix_ui::slider::*;
use leptos::prelude::*;

#[component]
pub fn SliderPage() -> impl IntoView {
    view! {
        <h1>"Slider"</h1>
        <p class="description">
            "An input where the user selects a value from within a given range."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="slider">
            <div class="hero-demo-card">
            <Slider default_value=vec![50.0] max=100.0 step=1.0 attr:class="demo-slider">
                <SliderTrack attr:class="demo-slider-track">
                    <SliderRange attr:class="demo-slider-range" />
                </SliderTrack>
                <SliderThumb attr:class="demo-slider-thumb" />
            </Slider>
            </div>
        </div>
        <div class="code-block">
            <div class="code-block-header">
                <span class="code-block-lang">"styles.css"</span>
            </div>
            <pre>{".SliderRoot {\n  position: relative;\n  display: flex;\n  align-items: center;\n  user-select: none;\n  touch-action: none;\n  width: 200px;\n  height: 20px;\n}\n\n.SliderTrack {\n  background-color: var(--black-a10);\n  position: relative;\n  flex-grow: 1;\n  border-radius: 9999px;\n  height: 3px;\n}\n\n.SliderRange {\n  position: absolute;\n  background-color: white;\n  border-radius: 9999px;\n  height: 100%;\n}\n\n.SliderThumb {\n  display: block;\n  width: 20px;\n  height: 20px;\n  background-color: white;\n  box-shadow: 0 2px 10px var(--black-a7);\n  border-radius: 10px;\n}\n.SliderThumb:hover {\n  background-color: var(--violet-3);\n}\n.SliderThumb:focus {\n  outline: none;\n  box-shadow: 0 0 0 5px var(--black-a12);\n}"}</pre>
        </div>

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports multiple thumbs."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports a minimum value between thumbs."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports touch and click on track to update value."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports Right to Left direction."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-slider"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_slider::*;\n\nview! {\n    <Slider default_value=vec![50.0]>\n        <SliderTrack>\n            <SliderRange />\n        </SliderTrack>\n        <SliderThumb />\n    </Slider>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a slider. It will render an input for each thumb when used within a form to ensure events propagate correctly."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"MaybeProp<Vec<f64>>"</td><td>"The controlled value of the slider."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<Vec<f64>>"</td><td>"The value of the slider when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<Vec<f64>>"</td><td>"Event handler called when the value changes."</td></tr>
                    <tr><td>"on_value_commit"</td><td>"Callback<Vec<f64>>"</td><td>"Event handler called when the user is done changing the value."</td></tr>
                    <tr><td>"min"</td><td>"MaybeProp<f64>"</td><td>"The minimum value for the range. Default: 0."</td></tr>
                    <tr><td>"max"</td><td>"MaybeProp<f64>"</td><td>"The maximum value for the range. Default: 100."</td></tr>
                    <tr><td>"step"</td><td>"MaybeProp<f64>"</td><td>"The stepping interval. Default: 1."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the slider."</td></tr>
                    <tr><td>"orientation"</td><td>"MaybeProp<String>"</td><td>"The orientation of the slider. Default: \"horizontal\"."</td></tr>
                    <tr><td>"dir"</td><td>"MaybeProp<Direction>"</td><td>"The reading direction of the slider. If omitted, inherits globally or assumes LTR."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="track">"Track"</h3>
        <p>"The track that contains the slider range."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="range">"Range"</h3>
        <p>"The range part. Must live inside Track."</p>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
                <tr><td>"[data-orientation]"</td><td>"\"horizontal\" | \"vertical\""</td></tr>
            </tbody>
        </table>

        <h3 id="thumb">"Thumb"</h3>
        <p>"A draggable thumb. You can render multiple thumbs."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"index"</td><td>"Option<usize>"</td><td>"The index of the thumb when using multiple thumbs. Default: 0."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/slider-multithumb" style="color:var(--text-link)">"Slider WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Increments/decrements by the step value depending on orientation."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Increments/decrements by the step value depending on orientation."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Increases the value by the step amount."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Decreases the value by the step amount."</td></tr>
                <tr><td><kbd>"PageUp"</kbd></td><td>"Increases the value by a larger step."</td></tr>
                <tr><td><kbd>"PageDown"</kbd></td><td>"Decreases the value by a larger step."</td></tr>
                <tr><td><kbd>"Home"</kbd></td><td>"Sets the value to its minimum."</td></tr>
                <tr><td><kbd>"End"</kbd></td><td>"Sets the value to its maximum."</td></tr>
            </tbody>
        </table>
    }
}
