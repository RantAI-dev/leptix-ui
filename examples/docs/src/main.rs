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
            <div class="layout">
                <Sidebar />
                <main class="main">
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
                </main>
            </div>
        </Router>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let components = vec![
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

    view! {
        <nav class="sidebar">
            <div class="sidebar-logo">
                <a href="/" style="text-decoration:none;color:inherit">
                    <span>"Leptix"</span>" UI"
                </a>
            </div>
            <div class="sidebar-section">"Getting Started"</div>
            <a href="/">"Introduction"</a>
            <div class="sidebar-section">"Components"</div>
            {components.into_iter().map(|(name, href)| {
                view! { <a href=href>{name}</a> }
            }).collect_view()}
        </nav>
    }
}
