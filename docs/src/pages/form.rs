use crate::HeroCodeBlock;
use leptix_ui::form::*;
use leptos::prelude::*;

#[component]
pub fn FormPage() -> impl IntoView {
    view! {
        <h1>"Form"</h1>
        <p class="description">
            "Collect information from your users using validation rules."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="form">
            <div class="hero-demo-card">
            <div class="demo-form">
                <Form>
                    <FormField name="email">
                        <FormLabel>"Email"</FormLabel>
                        <FormControl>
                            <input type="email" placeholder="you@example.com" />
                        </FormControl>
                        <FormMessage>"Please enter a valid email address."</FormMessage>
                    </FormField>

                    <FormField name="password">
                        <FormLabel>"Password"</FormLabel>
                        <FormControl>
                            <input type="password" placeholder="Enter password" />
                        </FormControl>
                        <FormMessage>"Password must be at least 8 characters."</FormMessage>
                    </FormField>

                    <button type="submit" style="margin-top:0.75rem;padding:0.5rem 1.25rem;border-radius:6px;background:var(--accent);color:#fff;border:none;cursor:pointer;font-weight:500;">
                        "Submit"
                    </button>
                </Form>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            usage_code="use leptix_ui::form::*;\n\nview! {\n    <Form>\n        <FormField name=\"email\">\n            <FormLabel>\"Email\"</FormLabel>\n            <FormControl>\n                <input type=\"email\" placeholder=\"you@example.com\" />\n            </FormControl>\n            <FormMessage>\"Please enter a valid email.\"</FormMessage>\n        </FormField>\n        <button type=\"submit\">\"Submit\"</button>\n    </Form>\n}"
            css_code=".FormRoot {\n  width: 300px;\n}\n\n.FormField {\n  display: grid;\n  margin-bottom: 10px;\n}\n\n.FormLabel {\n  font-size: 15px;\n  font-weight: 500;\n  line-height: 35px;\n  color: white;\n}\n\n.FormControl input {\n  width: 100%;\n  display: inline-flex;\n  align-items: center;\n  justify-content: center;\n  border-radius: 4px;\n  padding: 0 10px;\n  font-size: 15px;\n  line-height: 1;\n  color: white;\n  background-color: var(--black-a5);\n  box-shadow: 0 0 0 1px var(--black-a9);\n  height: 35px;\n}\n\n.FormMessage {\n  font-size: 13px;\n  color: white;\n  opacity: 0.8;\n}"
            css_modules_code=".Root {\n  width: 300px;\n}\n\n.Field {\n  display: grid;\n  margin-bottom: 10px;\n}\n\n.Label {\n  font-size: 15px;\n  font-weight: 500;\n  line-height: 35px;\n  color: white;\n}\n\n.Control input {\n  width: 100%;\n  display: inline-flex;\n  align-items: center;\n  justify-content: center;\n  border-radius: 4px;\n  padding: 0 10px;\n  font-size: 15px;\n  line-height: 1;\n  color: white;\n  background-color: var(--black-a5);\n  box-shadow: 0 0 0 1px var(--black-a9);\n  height: 35px;\n}\n\n.Message {\n  font-size: 13px;\n  color: white;\n  opacity: 0.8;\n}"
            tailwind_code="view! {\n    <Form class=\"w-[300px]\">\n        <FormField name=\"email\" class=\"grid mb-2.5\">\n            <FormLabel class=\"text-sm font-medium leading-[35px] text-white\">\n                \"Email\"\n            </FormLabel>\n            <FormControl>\n                <input type=\"email\" placeholder=\"you@example.com\"\n                    class=\"w-full inline-flex items-center justify-center rounded px-2.5 text-sm text-white bg-black/20 shadow-[0_0_0_1px_rgba(0,0,0,0.6)] h-[35px]\" />\n            </FormControl>\n            <FormMessage class=\"text-xs text-white/80\">\n                \"Please enter a valid email.\"\n            </FormMessage>\n        </FormField>\n        <button type=\"submit\" class=\"mt-3 px-5 py-2 rounded-md bg-violet-500 text-white font-medium\">\n            \"Submit\"\n        </button>\n    </Form>\n}"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Composes with standard form elements."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Supports built-in and custom validation."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Accessible error messages linked to fields."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-form"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_form::*;\n\nview! {\n    <Form>\n        <FormField name=\"email\">\n            <FormLabel />\n            <FormControl>\n                <input />\n            </FormControl>\n            <FormMessage />\n        </FormField>\n    </Form>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Wraps a native form element, providing context for field components."</p>

        <h3 id="field">"Field"</h3>
        <p>"Groups a label, control, and message for a single form field."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"name"</td><td>"&str"</td><td>"The name that associates the label, control, and message."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="label">"Label"</h3>
        <p>"A label element automatically associated with its control."</p>

        <h3 id="control">"Control"</h3>
        <p>"Wraps the native input element, linking it with the label and message."</p>

        <h3 id="message">"Message"</h3>
        <p>"Displays a validation message for the field."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"r#match"</td><td>"MaybeProp<Match>"</td><td>"The type of validation to display the message for."</td></tr>
                    <tr><td>"force_match"</td><td>"MaybeProp<bool>"</td><td>"Forces the message to show regardless of validity."</td></tr>
                </tbody>
            </table>
        </div>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Automatically links labels and error messages to controls via aria attributes."</p>
    }
}
