use crate::HeroCodeBlock;
use leptix_ui::scroll_area::*;
use leptos::prelude::*;

#[component]
pub fn ScrollAreaPage() -> impl IntoView {
    let items: Vec<String> = (1..=20).map(|i| format!("Item {i}")).collect();
    let items_view = items
        .into_iter()
        .map(|item| {
            view! {
                <div style="padding:0.5rem 0.75rem;border-bottom:1px solid var(--border);">
                    {item}
                </div>
            }
        })
        .collect_view();
    let items_stored = StoredValue::new(items_view);

    view! {
        <h1>"Scroll Area"</h1>
        <p class="description">
            "Augments native scroll functionality for custom, cross-browser styling."
        </p>

        // ---- Live Demo ----
        <div class="hero-container" data-component="scroll-area">
            <div class="hero-demo-card">
            <div class="demo-scroll-area">
                <ScrollArea>
                    <ScrollAreaViewport>
                        <div style="padding:0.75rem;">
                            {items_stored.get_value()}
                        </div>
                    </ScrollAreaViewport>
                    <ScrollAreaScrollbar orientation="vertical">
                        <ScrollAreaThumb />
                    </ScrollAreaScrollbar>
                </ScrollArea>
            </div>
            </div>
        </div>
        <HeroCodeBlock
            usage_code="use leptix_ui::scroll_area::*;\n\nview! {\n    <ScrollArea>\n        <ScrollAreaViewport>\n            <div>\"Scrollable content here...\"</div>\n        </ScrollAreaViewport>\n        <ScrollAreaScrollbar orientation=\"vertical\">\n            <ScrollAreaThumb />\n        </ScrollAreaScrollbar>\n    </ScrollArea>\n}"
            css_code=".ScrollAreaRoot {\n  width: 200px;\n  height: 225px;\n  border-radius: 4px;\n  overflow: hidden;\n  background-color: white;\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n\n.ScrollAreaViewport {\n  width: 100%;\n  height: 100%;\n  border-radius: inherit;\n}\n\n.ScrollAreaScrollbar {\n  display: flex;\n  user-select: none;\n  touch-action: none;\n  padding: 2px;\n  background: var(--black-a3);\n  transition: background 160ms ease-out;\n}\n\n.ScrollAreaScrollbar:hover {\n  background: var(--black-a5);\n}\n\n.ScrollAreaScrollbar[data-orientation=\"vertical\"] {\n  width: 10px;\n}\n\n.ScrollAreaThumb {\n  flex: 1;\n  background: var(--violet-9);\n  border-radius: 10px;\n  position: relative;\n}"
            css_modules_code=".Root {\n  width: 200px;\n  height: 225px;\n  border-radius: 4px;\n  overflow: hidden;\n  background-color: white;\n  box-shadow: 0 2px 10px var(--black-a7);\n}\n\n.Viewport {\n  width: 100%;\n  height: 100%;\n  border-radius: inherit;\n}\n\n.Scrollbar {\n  display: flex;\n  user-select: none;\n  touch-action: none;\n  padding: 2px;\n  background: var(--black-a3);\n  transition: background 160ms ease-out;\n}\n\n.Scrollbar:hover {\n  background: var(--black-a5);\n}\n\n.Scrollbar[data-orientation=\"vertical\"] {\n  width: 10px;\n}\n\n.Thumb {\n  flex: 1;\n  background: var(--violet-9);\n  border-radius: 10px;\n  position: relative;\n}"
            tailwind_code="view! {\n    <ScrollArea class=\"w-[200px] h-[225px] rounded overflow-hidden bg-white shadow-md\">\n        <ScrollAreaViewport class=\"w-full h-full rounded-[inherit]\">\n            <div>\"Scrollable content here...\"</div>\n        </ScrollAreaViewport>\n        <ScrollAreaScrollbar orientation=\"vertical\"\n            class=\"flex select-none touch-none p-0.5 bg-black/10 transition-colors hover:bg-black/20 w-2.5\">\n            <ScrollAreaThumb class=\"flex-1 bg-violet-500 rounded-full relative\" />\n        </ScrollAreaScrollbar>\n    </ScrollArea>\n}"
        />

        // ---- Highlights ----
        <div class="highlights">
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Scrollbar sits on top of the content, taking up no space."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Scrolling is native; no underlying position shifting via CSS transforms."</div>
            <div class="highlight-item"><span class="highlight-check">"✓"</span>"Shims pointer behaviors only when interacting with the scrollbar controls."</div>
        </div>

        // ---- Installation ----
        <h2 id="installation">"Installation"</h2>
        <p>"Install the component from your command line."</p>
        <div class="install-block">
            <span><span class="prompt">"$"</span>" cargo add leptix-scroll-area"</span>
        </div>

        // ---- Anatomy ----
        <h2 id="anatomy">"Anatomy"</h2>
        <p>"Import all parts and piece them together."</p>
        <div class="anatomy-block">{"use leptix_scroll_area::*;\n\nview! {\n    <ScrollArea>\n        <ScrollAreaViewport />\n        <ScrollAreaScrollbar orientation=\"vertical\">\n            <ScrollAreaThumb />\n        </ScrollAreaScrollbar>\n    </ScrollArea>\n}"}</div>

        // ---- API Reference ----
        <h2 id="api-reference">"API Reference"</h2>

        <h3 id="root">"Root"</h3>
        <p>"Contains all the parts of a scroll area."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"r#type"</td><td>"MaybeProp<ScrollAreaType>"</td><td>"Describes the nature of scrollbar visibility."</td></tr>
                    <tr><td>"scroll_hide_delay"</td><td>"MaybeProp<i32>"</td><td>"Delay in ms before hiding the scrollbar after the user stops scrolling. Default: 600."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="viewport">"Viewport"</h3>
        <p>"The viewport area for scrollable content."</p>

        <h3 id="scrollbar">"Scrollbar"</h3>
        <p>"The vertical or horizontal scrollbar. Add a ScrollAreaThumb inside."</p>
        <div class="props-table-wrapper">
            <table class="props-table">
                <thead><tr><th>"Prop"</th><th>"Type"</th><th>"Description"</th></tr></thead>
                <tbody>
                    <tr><td>"orientation"</td><td>"&str"</td><td>"The orientation of the scrollbar: \"vertical\" or \"horizontal\"."</td></tr>
                </tbody>
            </table>
        </div>

        <h3 id="thumb">"Thumb"</h3>
        <p>"The draggable thumb element inside the scrollbar."</p>

        // ---- Accessibility ----
        <h2 id="accessibility">"Accessibility"</h2>
        <p>"Scrolling via keyboard is supported by default because the component renders native scrollable elements."</p>
    }
}
