use std::sync::Arc;

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

/// Hook that detects interactions outside a given element and escape key presses.
///
/// Returns a node ref to attach to the dismissable layer element.
pub fn use_dismissable_layer(
    on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,
    on_focus_outside: Option<Callback<web_sys::FocusEvent>>,
    on_interact_outside: Option<Callback<web_sys::Event>>,
    on_dismiss: Option<Callback<()>>,
    disabled: Signal<bool>,
) -> AnyNodeRef {
    let node_ref = AnyNodeRef::new();

    // Handle escape key
    type KeyHandler = dyn Fn(web_sys::KeyboardEvent);
    let handle_keydown: Arc<SendWrapper<Closure<KeyHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::KeyboardEvent| {
            if disabled.get_untracked() {
                return;
            }
            if event.key() == "Escape" {
                if let Some(cb) = on_escape_key_down {
                    cb.run(event.clone());
                }
                if !event.default_prevented()
                    && let Some(dismiss) = on_dismiss
                {
                    dismiss.run(());
                }
            }
        }),
    ));

    // Handle pointer down outside
    type PointerHandler = dyn Fn(web_sys::PointerEvent);
    let handle_pointerdown: Arc<SendWrapper<Closure<PointerHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::PointerEvent| {
            if disabled.get_untracked() {
                return;
            }
            let target = event.target();
            let is_outside = target
                .as_ref()
                .and_then(|t| t.dyn_ref::<web_sys::Node>())
                .map(|target_node| {
                    node_ref
                        .get()
                        .map(|container| !container.contains(Some(target_node)))
                        .unwrap_or(false)
                })
                .unwrap_or(false);

            if is_outside {
                if let Some(cb) = on_pointer_down_outside {
                    cb.run(event.clone());
                }
                let event_as_event: &web_sys::Event = event.as_ref();
                if let Some(cb) = on_interact_outside {
                    cb.run(event_as_event.clone());
                }
                if !event.default_prevented()
                    && let Some(dismiss) = on_dismiss
                {
                    dismiss.run(());
                }
            }
        }),
    ));

    // Handle focus outside
    type FocusHandler = dyn Fn(web_sys::FocusEvent);
    let handle_focusin: Arc<SendWrapper<Closure<FocusHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::FocusEvent| {
            if disabled.get_untracked() {
                return;
            }
            let target = event.target();
            let is_outside = target
                .as_ref()
                .and_then(|t| t.dyn_ref::<web_sys::Node>())
                .map(|target_node| {
                    node_ref
                        .get()
                        .map(|container| !container.contains(Some(target_node)))
                        .unwrap_or(false)
                })
                .unwrap_or(false);

            if is_outside {
                if let Some(cb) = on_focus_outside {
                    cb.run(event.clone());
                }
                let event_as_event: &web_sys::Event = event.as_ref();
                if let Some(cb) = on_interact_outside {
                    cb.run(event_as_event.clone());
                }
                if !event.default_prevented()
                    && let Some(dismiss) = on_dismiss
                {
                    dismiss.run(());
                }
            }
        }),
    ));

    Effect::new({
        let handle_keydown = handle_keydown.clone();
        let handle_pointerdown = handle_pointerdown.clone();
        let handle_focusin = handle_focusin.clone();
        move |_| {
            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                return;
            };
            let _ = document.add_event_listener_with_callback(
                "keydown",
                (**handle_keydown).as_ref().unchecked_ref(),
            );
            let _ = document.add_event_listener_with_callback(
                "pointerdown",
                (**handle_pointerdown).as_ref().unchecked_ref(),
            );
            let _ = document.add_event_listener_with_callback(
                "focusin",
                (**handle_focusin).as_ref().unchecked_ref(),
            );
        }
    });

    on_cleanup(move || {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let _ = document.remove_event_listener_with_callback(
            "keydown",
            (**handle_keydown).as_ref().unchecked_ref(),
        );
        let _ = document.remove_event_listener_with_callback(
            "pointerdown",
            (**handle_pointerdown).as_ref().unchecked_ref(),
        );
        let _ = document.remove_event_listener_with_callback(
            "focusin",
            (**handle_focusin).as_ref().unchecked_ref(),
        );
    });

    node_ref
}
