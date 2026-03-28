use leptix_ui::slider::*;
use leptos::prelude::*;

#[component]
pub fn SliderPage() -> impl IntoView {
    view! {
        <h1>"Slider"</h1>
        <p class="subtitle">"An input where the user selects a value from within a given range by dragging a thumb."</p>

        <div class="features">
            <span class="feature">"Keyboard control"</span>
            <span class="feature">"Accessible"</span>
            <span class="feature">"RTL support"</span>
            <span class="feature">"Touch friendly"</span>
        </div>

        <h2>"Demo"</h2>
        <div class="demo-box">
            <div class="demo-preview">
                <Slider default_value=vec![50.0] max=100.0 step=1.0 attr:class="demo-slider">
                    <SliderTrack attr:class="demo-slider-track">
                        <SliderRange attr:class="demo-slider-range" />
                    </SliderTrack>
                    <SliderThumb attr:class="demo-slider-thumb" />
                </Slider>
            </div>
            <div class="demo-code">{"<Slider default_value=vec![50.0] max=100.0 step=1.0>\n  <SliderTrack>\n    <SliderRange />\n  </SliderTrack>\n  <SliderThumb />\n</Slider>"}</div>
        </div>

        <h2>"API Reference"</h2>

        <h3>"Slider"</h3>
        <p>"Contains all the parts of a slider. Provides context for the child components."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"value"</td><td>"MaybeProp<Vec<f64>>"</td><td>"Controlled value."</td></tr>
                <tr><td>"default_value"</td><td>"MaybeProp<Vec<f64>>"</td><td>"Uncontrolled initial value."</td></tr>
                <tr><td>"on_value_change"</td><td>"Callback<Vec<f64>>"</td><td>"Called on every value change during interaction."</td></tr>
                <tr><td>"on_value_commit"</td><td>"Callback<Vec<f64>>"</td><td>"Called when the user finishes interaction."</td></tr>
                <tr><td>"min"</td><td>"MaybeProp<f64>"</td><td>"Minimum value. Default: 0."</td></tr>
                <tr><td>"max"</td><td>"MaybeProp<f64>"</td><td>"Maximum value. Default: 100."</td></tr>
                <tr><td>"step"</td><td>"MaybeProp<f64>"</td><td>"Step increment. Default: 1."</td></tr>
                <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents user interaction."</td></tr>
                <tr><td>"orientation"</td><td>"MaybeProp<String>"</td><td>"The orientation. Default: \"horizontal\"."</td></tr>
                <tr><td>"dir"</td><td>"MaybeProp<Direction>"</td><td>"Reading direction for RTL support."</td></tr>
            </tbody>
        </table>

        <h3>"SliderTrack"</h3>
        <p>"The track that the thumb slides along."</p>

        <h3>"SliderRange"</h3>
        <p>"The filled range portion of the track."</p>

        <h3>"SliderThumb"</h3>
        <p>"A draggable thumb. Multiple thumbs can be used for range selection."</p>
        <table class="prop-table">
            <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td>"index"</td><td>"Option<usize>"</td><td>"The index of the thumb in a multi-thumb slider. Default: 0."</td></tr>
            </tbody>
        </table>

        <h2>"Accessibility"</h2>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Increments the value by the step amount."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Decrements the value by the step amount."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Increments the value by the step amount."</td></tr>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Decrements the value by the step amount."</td></tr>
                <tr><td><kbd>"PageUp"</kbd></td><td>"Increments the value by a larger step (10x)."</td></tr>
                <tr><td><kbd>"PageDown"</kbd></td><td>"Decrements the value by a larger step (10x)."</td></tr>
                <tr><td><kbd>"Home"</kbd></td><td>"Sets the value to the minimum."</td></tr>
                <tr><td><kbd>"End"</kbd></td><td>"Sets the value to the maximum."</td></tr>
            </tbody>
        </table>
    }
}
