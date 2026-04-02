use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

// Global counter for unique layer IDs.
static NEXT_LAYER_ID: AtomicU64 = AtomicU64::new(1);

fn next_layer_id() -> u64 {
    NEXT_LAYER_ID.fetch_add(1, Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// Global layer stack — shared across all dismissable layers in the app.
// In WASM this is single-threaded, so a thread_local RwSignal is safe.
// ---------------------------------------------------------------------------

thread_local! {
    static LAYER_STACK: std::cell::RefCell<Option<RwSignal<Vec<u64>>>> =
        const { std::cell::RefCell::new(None) };
}

fn get_layer_stack() -> RwSignal<Vec<u64>> {
    LAYER_STACK.with(|cell| {
        let mut opt = cell.borrow_mut();
        if let Some(signal) = *opt {
            signal
        } else {
            let signal = RwSignal::new(Vec::<u64>::new());
            *opt = Some(signal);
            signal
        }
    })
}

/// Hook that detects interactions outside a given element and escape key presses.
///
/// Implements a layer stack: only the **topmost** active layer responds to
/// Escape and outside-pointer/focus events. This prevents all open overlays
/// from closing simultaneously.
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
    let layer_id = next_layer_id();
    let stack = get_layer_stack();

    // Register this layer on the stack when enabled, remove when disabled/cleanup.
    Effect::new(move |_| {
        if disabled.get() {
            stack.update(|v| v.retain(|&id| id != layer_id));
        } else {
            stack.update(|v| {
                if !v.contains(&layer_id) {
                    v.push(layer_id);
                }
            });
        }
    });

    // --- Escape key: only fire for the topmost layer ---
    type KeyHandler = dyn Fn(web_sys::KeyboardEvent);
    let handle_keydown: Arc<SendWrapper<Closure<KeyHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::KeyboardEvent| {
            if disabled.get_untracked() {
                return;
            }
            let is_topmost =
                stack.with_untracked(|v| v.last().copied() == Some(layer_id));
            if !is_topmost {
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

    // --- Pointer down outside: only fire for topmost layer ---
    type PointerHandler = dyn Fn(web_sys::PointerEvent);
    let handle_pointerdown: Arc<SendWrapper<Closure<PointerHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::PointerEvent| {
            if disabled.get_untracked() {
                return;
            }
            // Ignore right-clicks — context menus should not dismiss layers.
            if event.button() == 2 {
                return;
            }
            let is_topmost =
                stack.with_untracked(|v| v.last().copied() == Some(layer_id));
            if !is_topmost {
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

    // --- Focus outside: only fire for topmost layer ---
    type FocusHandler = dyn Fn(web_sys::FocusEvent);
    let handle_focusin: Arc<SendWrapper<Closure<FocusHandler>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::FocusEvent| {
            if disabled.get_untracked() {
                return;
            }
            let is_topmost =
                stack.with_untracked(|v| v.last().copied() == Some(layer_id));
            if !is_topmost {
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

    // Attach document listeners.
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
        // Remove from layer stack.
        stack.update(|v| v.retain(|&id| id != layer_id));

        // Remove document listeners.
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
