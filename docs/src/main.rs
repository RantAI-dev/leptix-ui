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
    ("Avatar", "/avatar"),
    ("Checkbox", "/checkbox"),
    ("Collapsible", "/collapsible"),
    ("Dialog", "/dialog"),
    ("Label", "/label"),
    ("Progress", "/progress"),
    ("Separator", "/separator"),
    ("Slider", "/slider"),
    ("Switch", "/switch"),
    ("Tabs", "/tabs"),
    ("Toggle", "/toggle"),
];

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

#[component]
fn App() -> impl IntoView {
    let mobile_open = RwSignal::new(false);
    provide_context(mobile_open);

    // Close mobile menu on route change
    let location = use_location();
    Effect::new(move |_| {
        let _ = location.pathname.get();
        mobile_open.set(false);
    });

    view! {
        <Router>
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
                            <Route path=path!("/avatar") view=pages::avatar::AvatarPage />
                            <Route path=path!("/checkbox") view=pages::checkbox::CheckboxPage />
                            <Route path=path!("/collapsible") view=pages::collapsible::CollapsiblePage />
                            <Route path=path!("/dialog") view=pages::dialog::DialogPage />
                            <Route path=path!("/label") view=pages::label::LabelPage />
                            <Route path=path!("/progress") view=pages::progress::ProgressPage />
                            <Route path=path!("/separator") view=pages::separator::SeparatorPage />
                            <Route path=path!("/slider") view=pages::slider::SliderPage />
                            <Route path=path!("/switch") view=pages::switch::SwitchPage />
                            <Route path=path!("/tabs") view=pages::tabs::TabsPage />
                            <Route path=path!("/toggle") view=pages::toggle::TogglePage />
                        </Routes>
                        <DocsPagination />
                    </div>
                </div>
                <QuickNav />
            </div>
            <Footer />
        </Router>
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
