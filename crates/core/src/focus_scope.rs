use std::sync::Arc;

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

/// Hook that manages a focus scope — trapping focus, auto-focusing on mount,
/// and restoring focus on unmount.
pub fn use_focus_scope(
    trapped: Signal<bool>,
    do_loop: Signal<bool>,
    on_mount_auto_focus: Option<Callback<web_sys::Event>>,
    on_unmount_auto_focus: Option<Callback<web_sys::Event>>,
) -> AnyNodeRef {
    let container_ref = AnyNodeRef::new();
    let previously_focused: RwSignal<Option<SendWrapper<web_sys::Element>>> = RwSignal::new(None);

    // Auto-focus on mount
    Effect::new(move |_| {
        if let Some(container) = container_ref.get() {
            // Store the currently focused element to restore later
            let active = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.active_element());
            previously_focused.set(active.map(SendWrapper::new));

            // Fire auto-focus event
            if let Some(cb) = on_mount_auto_focus {
                let event =
                    web_sys::Event::new("focusScope.autoFocusOnMount").unwrap_or_else(|_| {
                        web_sys::Event::new("focus").expect("Event should be created")
                    });
                cb.run(event.clone());

                if !event.default_prevented() {
                    // Focus first focusable element
                    focus_first_in(&container);
                }
            } else {
                focus_first_in(&container);
            }
        }
    });

    // Handle tab key for focus trapping
    type KeyHandler = dyn Fn(web_sys::KeyboardEvent);
    let handle_keydown: Arc<SendWrapper<Closure<KeyHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::KeyboardEvent| {
            if event.key() != "Tab" {
                return;
            }

            let should_trap = trapped.get_untracked();
            let should_loop = do_loop.get_untracked();

            if !should_trap && !should_loop {
                return;
            }

            let Some(container) = container_ref.get() else {
                return;
            };

            let tabbables = get_tabbable_elements(&container);
            if tabbables.is_empty() {
                event.prevent_default();
                return;
            }

            let first = tabbables.first();
            let last = tabbables.last();
            let active = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.active_element());

            let shift = event.shift_key();

            if shift {
                // Shift+Tab: if on first element, wrap to last
                if active.as_ref() == first.map(|e| e.unchecked_ref::<web_sys::Element>()) {
                    event.prevent_default();
                    if let Some(last) = last {
                        let _ = last.focus();
                    }
                }
            } else {
                // Tab: if on last element, wrap to first
                if active.as_ref() == last.map(|e| e.unchecked_ref::<web_sys::Element>()) {
                    event.prevent_default();
                    if let Some(first) = first {
                        let _ = first.focus();
                    }
                }
            }
        }),
    ));

    // Handle focus leaving the container when trapped
    type FocusHandler = dyn Fn(web_sys::FocusEvent);
    let handle_focusout: Arc<SendWrapper<Closure<FocusHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::FocusEvent| {
            if !trapped.get_untracked() {
                return;
            }

            let Some(container) = container_ref.get() else {
                return;
            };

            let related_target = event
                .related_target()
                .and_then(|t| t.dyn_into::<web_sys::Node>().ok());

            // If focus is leaving the container, pull it back
            if let Some(related) = related_target
                && !container.contains(Some(&related))
            {
                // Focus went outside - bring it back
                let tabbables = get_tabbable_elements(&container);
                if let Some(first) = tabbables.first() {
                    let _ = first.focus();
                }
            }
        }),
    ));

    Effect::new({
        let handle_keydown = handle_keydown.clone();
        let handle_focusout = handle_focusout.clone();
        move |_| {
            if let Some(container) = container_ref.get() {
                let _ = container.add_event_listener_with_callback(
                    "keydown",
                    (**handle_keydown).as_ref().unchecked_ref(),
                );
                let _ = container.add_event_listener_with_callback(
                    "focusout",
                    (**handle_focusout).as_ref().unchecked_ref(),
                );
            }
        }
    });

    // Restore focus on unmount
    on_cleanup(move || {
        if let Some(container) = container_ref.get() {
            let _ = container.remove_event_listener_with_callback(
                "keydown",
                (**handle_keydown).as_ref().unchecked_ref(),
            );
            let _ = container.remove_event_listener_with_callback(
                "focusout",
                (**handle_focusout).as_ref().unchecked_ref(),
            );
        }

        if let Some(cb) = on_unmount_auto_focus {
            let event = web_sys::Event::new("focusScope.autoFocusOnUnmount")
                .unwrap_or_else(|_| web_sys::Event::new("focus").expect("Event should be created"));
            cb.run(event.clone());

            if !event.default_prevented()
                && let Some(prev) = previously_focused.get_untracked()
                && let Ok(el) = (*prev).clone().dyn_into::<web_sys::HtmlElement>()
            {
                let _ = el.focus();
            }
        } else if let Some(prev) = previously_focused.get_untracked()
            && let Ok(el) = (*prev).clone().dyn_into::<web_sys::HtmlElement>()
        {
            let _ = el.focus();
        }
    });

    container_ref
}

fn focus_first_in(container: &web_sys::Element) {
    let tabbables = get_tabbable_elements(container);
    if let Some(first) = tabbables.first() {
        let _ = first.focus();
    }
}

fn get_tabbable_elements(container: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
    let selector = r#"a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"]):not([disabled])"#;

    let Ok(nodes) = container.query_selector_all(selector) else {
        return vec![];
    };

    let window = web_sys::window();

    let mut result = vec![];
    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i)
            && let Ok(el) = node.dyn_into::<web_sys::HtmlElement>()
        {
            // Skip elements inside inert subtrees
            if el.closest("[inert]").ok().flatten().is_some() {
                continue;
            }

            // Use getComputedStyle for accurate visibility check
            let is_visible = window
                .as_ref()
                .and_then(|w| w.get_computed_style(&el).ok().flatten())
                .map(|style| {
                    let display = style.get_property_value("display").unwrap_or_default();
                    let visibility = style.get_property_value("visibility").unwrap_or_default();
                    display != "none" && visibility != "hidden"
                })
                .unwrap_or(true);

            if is_visible {
                result.push(el);
            }
        }
    }
    result
}
