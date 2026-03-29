mod pages;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_location;
use leptos_router::path;
use web_sys::wasm_bindgen::JsCast;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

// ---------------------------------------------------------------------------
// Page order for pagination
// ---------------------------------------------------------------------------

const PAGE_ORDER: &[(&str, &str)] = &[
    ("Introduction", "/"),
    ("Accordion", "/accordion"),
    ("Alert Dialog", "/alert-dialog"),
    ("Aspect Ratio", "/aspect-ratio"),
    ("Avatar", "/avatar"),
    ("Checkbox", "/checkbox"),
    ("Collapsible", "/collapsible"),
    ("Context Menu", "/context-menu"),
    ("Dialog", "/dialog"),
    ("Dropdown Menu", "/dropdown-menu"),
    ("Form", "/form"),
    ("Hover Card", "/hover-card"),
    ("Label", "/label"),
    ("Menubar", "/menubar"),
    ("Navigation Menu", "/navigation-menu"),
    ("OTP Field", "/otp-field"),
    ("Password Toggle", "/password-toggle"),
    ("Popover", "/popover"),
    ("Progress", "/progress"),
    ("Radio Group", "/radio-group"),
    ("Scroll Area", "/scroll-area"),
    ("Select", "/select"),
    ("Separator", "/separator"),
    ("Slider", "/slider"),
    ("Switch", "/switch"),
    ("Tabs", "/tabs"),
    ("Toast", "/toast"),
    ("Toggle", "/toggle"),
    ("Toggle Group", "/toggle-group"),
    ("Toolbar", "/toolbar"),
    ("Tooltip", "/tooltip"),
    ("Accessible Icon", "/accessible-icon"),
    ("Direction Provider", "/direction-provider"),
    ("Portal", "/portal"),
    ("Visually Hidden", "/visually-hidden"),
];

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

#[component]
fn App() -> impl IntoView {
    let mobile_open = RwSignal::new(false);
    provide_context(mobile_open);

    view! {
        <Router>
            <AppInner />
        </Router>
    }
}

/// Inner shell — lives inside `<Router>` so `use_location()` works.
#[component]
fn AppInner() -> impl IntoView {
    let mobile_open = expect_context::<RwSignal<bool>>();

    // Close mobile menu on route change
    let location = use_location();
    Effect::new(move |_| {
        let _ = location.pathname.get();
        mobile_open.set(false);
    });

    view! {
        <Header />
        <div class="layout">
            <div class="sidebar-overlay"
                class:open=move || mobile_open.get()
                on:click=move |_| mobile_open.set(false)
            />
            <Sidebar />
            <div class="main-content">
                <div class="doc-page">
                        <Routes fallback=|| view! { <p>"Page not found"</p> }>
                            <Route path=path!("/") view=pages::home::HomePage />
                            <Route path=path!("/accordion") view=pages::accordion::AccordionPage />
                            <Route path=path!("/alert-dialog") view=pages::alert_dialog::AlertDialogPage />
                            <Route path=path!("/aspect-ratio") view=pages::aspect_ratio::AspectRatioPage />
                            <Route path=path!("/avatar") view=pages::avatar::AvatarPage />
                            <Route path=path!("/checkbox") view=pages::checkbox::CheckboxPage />
                            <Route path=path!("/collapsible") view=pages::collapsible::CollapsiblePage />
                            <Route path=path!("/context-menu") view=pages::context_menu::ContextMenuPage />
                            <Route path=path!("/dialog") view=pages::dialog::DialogPage />
                            <Route path=path!("/dropdown-menu") view=pages::dropdown_menu::DropdownMenuPage />
                            <Route path=path!("/form") view=pages::form::FormPage />
                            <Route path=path!("/hover-card") view=pages::hover_card::HoverCardPage />
                            <Route path=path!("/label") view=pages::label::LabelPage />
                            <Route path=path!("/menubar") view=pages::menubar::MenubarPage />
                            <Route path=path!("/navigation-menu") view=pages::navigation_menu::NavigationMenuPage />
                            <Route path=path!("/otp-field") view=pages::otp_field::OtpFieldPage />
                            <Route path=path!("/password-toggle") view=pages::password_toggle::PasswordTogglePage />
                            <Route path=path!("/popover") view=pages::popover::PopoverPage />
                            <Route path=path!("/progress") view=pages::progress::ProgressPage />
                            <Route path=path!("/radio-group") view=pages::radio_group::RadioGroupPage />
                            <Route path=path!("/scroll-area") view=pages::scroll_area::ScrollAreaPage />
                            <Route path=path!("/select") view=pages::select::SelectPage />
                            <Route path=path!("/separator") view=pages::separator::SeparatorPage />
                            <Route path=path!("/slider") view=pages::slider::SliderPage />
                            <Route path=path!("/switch") view=pages::switch::SwitchPage />
                            <Route path=path!("/tabs") view=pages::tabs::TabsPage />
                            <Route path=path!("/toast") view=pages::toast::ToastPage />
                            <Route path=path!("/toggle") view=pages::toggle::TogglePage />
                            <Route path=path!("/toggle-group") view=pages::toggle_group::ToggleGroupPage />
                            <Route path=path!("/toolbar") view=pages::toolbar::ToolbarPage />
                            <Route path=path!("/tooltip") view=pages::tooltip::TooltipPage />
                            <Route path=path!("/accessible-icon") view=pages::accessible_icon::AccessibleIconPage />
                            <Route path=path!("/direction-provider") view=pages::direction_provider::DirectionProviderPage />
                            <Route path=path!("/portal") view=pages::portal::PortalPage />
                            <Route path=path!("/visually-hidden") view=pages::visually_hidden::VisuallyHiddenPage />
                        </Routes>
                        <DocsPagination />
                    </div>
                </div>
                <QuickNav />
            </div>
            <Footer />
    }
}

// ---------------------------------------------------------------------------
// Header with theme toggle + mobile menu button
// ---------------------------------------------------------------------------

#[component]
fn Header() -> impl IntoView {
    let mobile_open = expect_context::<RwSignal<bool>>();
    let (is_dark, set_dark) = signal(false);

    let toggle_theme = move |_| {
        let new_dark = !is_dark.get();
        set_dark.set(new_dark);
        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            let _ = el.set_attribute("data-theme", if new_dark { "dark" } else { "light" });
        }
    };

    view! {
        <header class="header">
            <div class="header-left">
                <button class="mobile-menu-btn"
                    on:click=move |_| mobile_open.set(!mobile_open.get())
                    aria-label="Toggle menu"
                >
                    {move || if mobile_open.get() { "✕" } else { "☰" }}
                </button>
                <a href="/" class="header-logo">
                    <img src="public/logo/favicon-32x32.png" alt="" width="22" height="22" style="border-radius:5px" />
                    "Leptix"
                </a>
                <nav class="header-nav">
                    <a href="/" class="active">"Primitives"</a>
                </nav>
            </div>
            <div class="header-right">
                <button class="theme-toggle" on:click=toggle_theme aria-label="Toggle theme">
                    {move || if is_dark.get() { "☀" } else { "☾" }}
                </button>
            </div>
        </header>
    }
}

// ---------------------------------------------------------------------------
// Sidebar with active route detection
// ---------------------------------------------------------------------------

#[component]
fn Sidebar() -> impl IntoView {
    let mobile_open = expect_context::<RwSignal<bool>>();
    let location = use_location();

    let components: Vec<(&str, &str, Option<&str>)> = vec![
        ("Accordion", "/accordion", None),
        ("Alert Dialog", "/alert-dialog", None),
        ("Aspect Ratio", "/aspect-ratio", None),
        ("Avatar", "/avatar", None),
        ("Checkbox", "/checkbox", None),
        ("Collapsible", "/collapsible", None),
        ("Context Menu", "/context-menu", None),
        ("Dialog", "/dialog", None),
        ("Dropdown Menu", "/dropdown-menu", None),
        ("Form", "/form", None),
        ("Hover Card", "/hover-card", None),
        ("Label", "/label", None),
        ("Menubar", "/menubar", None),
        ("Navigation Menu", "/navigation-menu", None),
        ("OTP Field", "/otp-field", Some("Preview")),
        ("Password Toggle", "/password-toggle", Some("Preview")),
        ("Popover", "/popover", None),
        ("Progress", "/progress", None),
        ("Radio Group", "/radio-group", None),
        ("Scroll Area", "/scroll-area", None),
        ("Select", "/select", None),
        ("Separator", "/separator", None),
        ("Slider", "/slider", None),
        ("Switch", "/switch", None),
        ("Tabs", "/tabs", None),
        ("Toast", "/toast", None),
        ("Toggle", "/toggle", None),
        ("Toggle Group", "/toggle-group", None),
        ("Toolbar", "/toolbar", None),
        ("Tooltip", "/tooltip", None),
    ];

    let utilities: Vec<(&str, &str)> = vec![
        ("Accessible Icon", "/accessible-icon"),
        ("Direction Provider", "/direction-provider"),
        ("Portal", "/portal"),
        ("Visually Hidden", "/visually-hidden"),
    ];

    view! {
        <nav class="sidebar" class:open=move || mobile_open.get()>
            <div class="nav-section">"Overview"</div>
            <a href="/" class="nav-link"
                class:active=move || location.pathname.get() == "/"
            >"Introduction"</a>

            <div class="nav-section">"Components"</div>
            {components.into_iter().map(|(title, href, badge)| {
                let href_owned = href.to_string();
                view! {
                    <a href=href class="nav-link"
                        class:active=move || location.pathname.get() == href_owned
                    >
                        {title}
                        {badge.map(|b| view! { <span class="nav-badge">{b}</span> })}
                    </a>
                }
            }).collect_view()}

            <div class="nav-section">"Utilities"</div>
            {utilities.into_iter().map(|(title, href)| {
                let href_owned = href.to_string();
                view! {
                    <a href=href class="nav-link"
                        class:active=move || location.pathname.get() == href_owned
                    >{title}</a>
                }
            }).collect_view()}
        </nav>
    }
}

// ---------------------------------------------------------------------------
// Quick Nav (right sidebar TOC)
// ---------------------------------------------------------------------------

#[component]
fn QuickNav() -> impl IntoView {
    let location = use_location();
    let (headings, set_headings) = signal::<Vec<(String, String, bool)>>(vec![]);

    // Scan headings after each navigation
    Effect::new(move |_| {
        let _ = location.pathname.get();
        // Defer to next frame so page content has rendered
        if let Some(window) = web_sys::window() {
            let cb = wasm_bindgen::closure::Closure::once(move || {
                if let Some(document) = web_sys::window().and_then(|w| w.document())
                    && let Ok(nodes) =
                        document.query_selector_all(".doc-page h2[id], .doc-page h3[id]")
                {
                    let mut items = vec![];
                    for i in 0..nodes.length() {
                        if let Some(node) = nodes.item(i)
                            && let Ok(el) = node.dyn_into::<web_sys::HtmlElement>()
                        {
                            let id = el.id();
                            let text = el.inner_text();
                            let is_h3 = el.tag_name() == "H3";
                            if !id.is_empty() {
                                items.push((id, text, is_h3));
                            }
                        }
                    }
                    set_headings.set(items);
                }
            });
            let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    view! {
        <div class="quick-nav">
            <Show when=move || !headings.get().is_empty()>
                <div class="quick-nav-title">"On this page"</div>
                {move || headings.get().into_iter().map(|(id, text, is_h3)| {
                    let href = format!("#{}", id);
                    view! {
                        <a href=href class:indent=is_h3>{text}</a>
                    }
                }).collect_view()}
            </Show>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

#[component]
fn DocsPagination() -> impl IntoView {
    let location = use_location();

    let nav = move || {
        let path = location.pathname.get();
        let idx = PAGE_ORDER.iter().position(|(_, p)| *p == path.as_str());
        let prev = idx.and_then(|i| if i > 0 { PAGE_ORDER.get(i - 1) } else { None });
        let next = idx.and_then(|i| PAGE_ORDER.get(i + 1));
        (prev.copied(), next.copied())
    };

    view! {
        <div class="pagination">
            <div>
                {move || {
                    let (prev, _) = nav();
                    prev.map(|(name, href)| view! {
                        <a href=href>
                            <div class="direction">"← Previous"</div>
                            <div class="page-title">{name}</div>
                        </a>
                    })
                }}
            </div>
            <div style="text-align:right">
                {move || {
                    let (_, next) = nav();
                    next.map(|(name, href)| view! {
                        <a href=href>
                            <div class="direction">"Next →"</div>
                            <div class="page-title">{name}</div>
                        </a>
                    })
                }}
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Footer
// ---------------------------------------------------------------------------

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="footer-brand">
                <div class="footer-brand-name">"Leptix"</div>
                <div class="footer-brand-desc">
                    "Unstyled, accessible UI primitives for Leptos. Based on Radix UI."
                </div>
            </div>
            <div class="footer-links">
                <div class="footer-col">
                    <h4>"Docs"</h4>
                    <a href="/">"Introduction"</a>
                    <a href="/dialog">"Dialog"</a>
                    <a href="/tabs">"Tabs"</a>
                    <a href="/accordion">"Accordion"</a>
                </div>
                <div class="footer-col">
                    <h4>"Community"</h4>
                    <a href="https://github.com/rantai-dev/leptix" target="_blank">"GitHub"</a>
                    <a href="https://crates.io/crates/leptix-ui" target="_blank">"crates.io"</a>
                </div>
            </div>
        </footer>
    }
}

// ---------------------------------------------------------------------------
// Reusable CodeBlock component with copy button
// ---------------------------------------------------------------------------

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(into, optional)] lang: Option<String>,
) -> impl IntoView {
    let (copied, set_copied) = signal(false);
    let code_for_copy = code.clone();

    let on_copy = move |_| {
        if let Some(window) = web_sys::window() {
            let nav = window.navigator();
            let clipboard = nav.clipboard();
            let _ = clipboard.write_text(&code_for_copy);
            set_copied.set(true);
            // Reset after 2 seconds
            let cb = wasm_bindgen::closure::Closure::once(move || {
                set_copied.set(false);
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                2000,
            );
            cb.forget();
        }
    };

    let lang_display = lang.clone().unwrap_or_else(|| "rust".into());

    view! {
        <div class="code-block standalone">
            <div class="code-block-header">
                <span class="code-block-lang">{lang_display}</span>
                <button class="copy-btn" class:copied=move || copied.get() on:click=on_copy>
                    {move || if copied.get() { "Copied!" } else { "Copy" }}
                </button>
            </div>
            <pre>{code}</pre>
        </div>
    }
}

// ---------------------------------------------------------------------------
// HeroCodeBlock — multi-tab code display (usage.rs + CSS/Modules/Tailwind)
// ---------------------------------------------------------------------------

#[component]
pub fn HeroCodeBlock(
    #[prop(into)] css_usage: String,
    #[prop(into)] css_styles: String,
    #[prop(into)] modules_usage: String,
    #[prop(into)] modules_styles: String,
    #[prop(into)] tailwind_usage: String,
    #[prop(into)] tailwind_config: String,
) -> impl IntoView {
    let (active_tab, set_active_tab) = signal("usage");
    let (css_mode, set_css_mode) = signal("css");
    let (copied, set_copied) = signal(false);

    let css_u = StoredValue::new(css_usage);
    let css_s = StoredValue::new(css_styles);
    let mod_u = StoredValue::new(modules_usage);
    let mod_s = StoredValue::new(modules_styles);
    let tw_u = StoredValue::new(tailwind_usage);
    let tw_c = StoredValue::new(tailwind_config);

    let styles_filename = move || match css_mode.get() {
        "css-modules" => "styles.module.css",
        "tailwind" => "tailwind.config.ts",
        _ => "styles.css",
    };

    let current_code = move || match (active_tab.get(), css_mode.get()) {
        ("usage", "css") => css_u.get_value(),
        ("usage", "css-modules") => mod_u.get_value(),
        ("usage", "tailwind") => tw_u.get_value(),
        (_, "css") => css_s.get_value(),
        (_, "css-modules") => mod_s.get_value(),
        (_, "tailwind") => tw_c.get_value(),
        _ => css_u.get_value(),
    };

    let on_copy = move |_| {
        let code = current_code();
        if let Some(window) = web_sys::window() {
            let nav = window.navigator();
            let clipboard = nav.clipboard();
            let _ = clipboard.write_text(&code);
            set_copied.set(true);
            let cb = wasm_bindgen::closure::Closure::once(move || {
                set_copied.set(false);
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                2000,
            );
            cb.forget();
        }
    };

    view! {
        <div class="hero-code-block">
            <div class="hero-code-tabs">
                <div class="hero-code-tabs-left">
                    <button
                        class="hero-code-tab"
                        class:active=move || active_tab.get() == "usage"
                        on:click=move |_| set_active_tab.set("usage")
                    >
                        "usage.rs"
                    </button>
                    <button
                        class="hero-code-tab"
                        class:active=move || active_tab.get() == "styles"
                        on:click=move |_| set_active_tab.set("styles")
                    >
                        {styles_filename}
                    </button>
                </div>
                <div class="hero-code-tabs-right">
                    <select
                        class="hero-code-mode"
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            set_css_mode.set(match val.as_str() {
                                "css-modules" => "css-modules",
                                "tailwind" => "tailwind",
                                _ => "css",
                            });
                        }
                    >
                        <option value="css" selected=move || css_mode.get() == "css">"CSS"</option>
                        <option value="css-modules" selected=move || css_mode.get() == "css-modules">"CSS Modules"</option>
                        <option value="tailwind" selected=move || css_mode.get() == "tailwind">"Tailwind CSS"</option>
                    </select>
                    <button class="copy-btn always-visible" class:copied=move || copied.get() on:click=on_copy>
                        {move || if copied.get() { "Copied!" } else { "Copy" }}
                    </button>
                </div>
            </div>
            <pre>{current_code}</pre>
        </div>
    }
}
