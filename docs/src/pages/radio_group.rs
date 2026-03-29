use crate::HeroCodeBlock;
use leptix_ui::radio_group::*;
use leptos::prelude::*;

#[component]
pub fn RadioGroupPage() -> impl IntoView {
    view! {
        <h1>"Radio Group"</h1>
        <p class="description">
            "A set of checkable buttons — known as radio buttons — where no more than one of the buttons can be checked at a time."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="radio-group">
            <div class="hero-demo-card">
            <div class="demo-radio-group">
                <RadioGroup default_value="default".to_string()>
                    <RadioGroupItem value="default">
                        <RadioGroupIndicator />
                    </RadioGroupItem>
                    <label>"Default"</label>

                    <RadioGroupItem value="comfortable">
                        <RadioGroupIndicator />
                    </RadioGroupItem>
                    <label>"Comfortable"</label>

                    <RadioGroupItem value="compact">
                        <RadioGroupIndicator />
                    </RadioGroupItem>
                    <label>"Compact"</label>
                </RadioGroup>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            usage_code="use leptix_radio_group::*;\n\nview! {\n    <RadioGroup class=\"RadioGroupRoot\" default_value=\"default\".to_string()>\n        <RadioGroupItem class=\"RadioGroupItem\" value=\"default\">\n            <RadioGroupIndicator class=\"RadioGroupIndicator\" />\n        </RadioGroupItem>\n        <label>\"Default\"</label>\n\n        <RadioGroupItem class=\"RadioGroupItem\" value=\"comfortable\">\n            <RadioGroupIndicator class=\"RadioGroupIndicator\" />\n        </RadioGroupItem>\n        <label>\"Comfortable\"</label>\n    </RadioGroup>\n}"
            css_code=".RadioGroupRoot {\n  display: flex;\n  flex-direction: column;\n  gap: 10px;\n}\n\n.RadioGroupItem {\n  all: unset;\n  background-color: white;\n  width: 25px;\n  height: 25px;\n  border-radius: 100%;\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n.RadioGroupItem:hover {\n  background-color: var(--violet-3);\n}\n.RadioGroupItem:focus {\n  box-shadow: 0 0 0 2px black;\n}\n\n.RadioGroupIndicator {\n  display: flex;\n  align-items: center;\n  justify-content: center;\n  width: 100%;\n  height: 100%;\n  position: relative;\n}\n.RadioGroupIndicator::after {\n  content: '';\n  display: block;\n  width: 11px;\n  height: 11px;\n  border-radius: 50%;\n  background-color: var(--violet-11);\n}"
            css_modules_code=".Root {\n  display: flex;\n  flex-direction: column;\n  gap: 10px;\n}\n\n.Item {\n  all: unset;\n  background-color: white;\n  width: 25px;\n  height: 25px;\n  border-radius: 100%;\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n.Item:hover {\n  background-color: var(--violet-3);\n}\n.Item:focus {\n  box-shadow: 0 0 0 2px black;\n}\n\n.Indicator {\n  display: flex;\n  align-items: center;\n  justify-content: center;\n  width: 100%;\n  height: 100%;\n  position: relative;\n}\n.Indicator::after {\n  content: '';\n  display: block;\n  width: 11px;\n  height: 11px;\n  border-radius: 50%;\n  background-color: var(--violet-11);\n}"
            tailwind_code="<RadioGroup class=\"flex flex-col gap-2.5\" default_value=\"default\".to_string()>\n    <RadioGroupItem class=\"bg-white w-[25px] h-[25px] rounded-full shadow-md hover:bg-violet-100 focus:shadow-[0_0_0_2px_black]\" value=\"default\">\n        <RadioGroupIndicator class=\"flex items-center justify-center w-full h-full relative after:block after:w-[11px] after:h-[11px] after:rounded-full after:bg-violet-700\" />\n    </RadioGroupItem>\n    <label>\"Default\"</label>\n</RadioGroup>"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Full keyboard navigation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports horizontal/vertical orientation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Can be controlled or uncontrolled."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-radio-group"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_radio_group::*;\n\nview! {\n    <RadioGroup default_value=\"default\".to_string()>\n        <RadioGroupItem value=\"default\">\n            <RadioGroupIndicator />\n        </RadioGroupItem>\n    </RadioGroup>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a radio group."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"MaybeProp<String>"</td><td>"The controlled value of the radio item to check."</td></tr>
                    <tr><td>"default_value"</td><td>"MaybeProp<String>"</td><td>"The value of the radio item that should be checked when initially rendered."</td></tr>
                    <tr><td>"on_value_change"</td><td>"Callback<String>"</td><td>"Event handler called when the value changes."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with radio items."</td></tr>
                    <tr><td>"name"</td><td>"MaybeProp<String>"</td><td>"The name of the group. Submitted with its owning form as part of a name/value pair."</td></tr>
                    <tr><td>"required"</td><td>"MaybeProp<bool>"</td><td>"When true, indicates that the user must check a radio item before the owning form can be submitted."</td></tr>
                    <tr><td>"orientation"</td><td>"MaybeProp<String>"</td><td>"The orientation of the component. Default: \"vertical\"."</td></tr>
                    <tr><td>"loop"</td><td>"MaybeProp<bool>"</td><td>"When true, keyboard navigation will loop from last item to first and vice versa. Default: true."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        <h3 id="item">"Item"</h3>
        <p>"An item in the group that can be checked."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"value"</td><td>"String"</td><td>"The value given as data when submitted with a name."</td></tr>
                    <tr><td>"disabled"</td><td>"MaybeProp<bool>"</td><td>"When true, prevents the user from interacting with the radio item."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"checked\" | \"unchecked\""</td></tr>
                <tr><td>"[data-disabled]"</td><td>"Present when disabled"</td></tr>
            </tbody>
        </table>

        <h3 id="indicator">"Indicator"</h3>
        <p>"Renders when the radio item is in a checked state. You can style this element directly, or you can use it as a wrapper to put an icon into, or both."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"force_mount"</td><td>"MaybeProp<bool>"</td><td>"Used to force mounting when more control is needed."</td></tr>
                </tbody>
            </table>
        </div>
        <table class="data-attrs-table">
            <thead><tr><th>"Data Attribute"</th><th>"Values"</th></tr></thead>
            <tbody>
                <tr><td>"[data-state]"</td><td>"\"checked\" | \"unchecked\""</td></tr>
            </tbody>
        </table>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Adheres to the "<a href="https://www.w3.org/WAI/ARIA/apg/patterns/radio" style="color:var(--text-link)">"Radio Group WAI-ARIA design pattern"</a>"."</p>

        <h3>"Keyboard Interactions"</h3>
        <table class="kbd-table">
            <thead><tr><th>"Key"</th><th>"Description"</th></tr></thead>
            <tbody>
                <tr><td><kbd>"ArrowDown"</kbd></td><td>"Moves focus to the next radio item in the group."</td></tr>
                <tr><td><kbd>"ArrowRight"</kbd></td><td>"Moves focus to the next radio item in the group."</td></tr>
                <tr><td><kbd>"ArrowUp"</kbd></td><td>"Moves focus to the previous radio item in the group."</td></tr>
                <tr><td><kbd>"ArrowLeft"</kbd></td><td>"Moves focus to the previous radio item in the group."</td></tr>
                <tr><td><kbd>"Tab"</kbd></td><td>"Moves focus to the checked radio item or the first radio item in the group."</td></tr>
            </tbody>
        </table>
    }
}
