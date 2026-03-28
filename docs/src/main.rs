mod pages;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <header class="header">
                <a href="/" class="header-logo">
                    <img src="/public/logo.svg" alt="" width="22" height="22" />
                    "Leptix"
                </a>
                <nav class="header-nav">
                    <a href="/" class="active">"Primitives"</a>
                </nav>
            </header>

            <div class="layout">
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
                    </div>
                </div>
            </div>
        </Router>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
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
        <nav class="sidebar">
            <div class="nav-section">"Overview"</div>
            <a href="/" class="nav-link">"Introduction"</a>

            <div class="nav-section">"Components"</div>
            {components.into_iter().map(|(title, href, badge)| {
                view! {
                    <a href=href class="nav-link">
                        {title}
                        {badge.map(|b| view! { <span class="nav-badge">{b}</span> })}
                    </a>
                }
            }).collect_view()}

            <div class="nav-section">"Utilities"</div>
            {utilities.into_iter().map(|(title, href)| {
                view! { <a href=href class="nav-link">{title}</a> }
            }).collect_view()}
        </nav>
    }
}
