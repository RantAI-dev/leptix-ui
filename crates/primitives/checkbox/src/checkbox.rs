use std::fmt::{Display, Formatter};
use std::sync::Arc;

use leptix_core::compose_refs::use_composed_refs;
use leptix_core::presence::use_presence;
use leptix_core::primitive::{Primitive, compose_callbacks};
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptix_core::use_previous::use_previous;
use leptix_core::use_size::use_size;
use leptos::{context::Provider, ev::KeyboardEvent, ev::MouseEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CheckedState {
    False,
    True,
    Indeterminate,
}

impl Display for CheckedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CheckedState::False => "false",
                CheckedState::True => "true",
                CheckedState::Indeterminate => "indeterminate",
            }
        )
    }
}

#[derive(Clone, Debug)]
struct CheckboxContextValue {
    state: Signal<CheckedState>,
    disabled: Signal<bool>,
}

#[component]
pub fn Checkbox(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] default_checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<CheckedState>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_keydown: Option<Callback<KeyboardEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let name = Signal::derive(move || name.get());
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let value = Signal::derive(move || value.get().unwrap_or("on".into()));

    let button_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, button_ref]);

    let is_form_control = Signal::derive(move || {
        button_ref
            .get()
            .and_then(|button| button.closest("form").ok())
            .flatten()
            .is_some()
    });

    let (checked, set_checked) = use_controllable_state(UseControllableStateParams {
        prop: checked,
        on_change: on_checked_change.map(|on_checked_change| {
            Callback::new(move |value| {
                if let Some(value) = value {
                    on_checked_change.run(value);
                }
            })
        }),
        default_prop: default_checked,
    });
    let checked = Signal::derive(move || checked.get().unwrap_or(CheckedState::False));

    let initial_checked_state = RwSignal::new(checked.get_untracked());

    type ResetHandler = dyn Fn(web_sys::Event);
    let handle_reset: Arc<SendWrapper<Closure<ResetHandler>>> =
        Arc::new(SendWrapper::new(Closure::new(move |_: web_sys::Event| {
            set_checked.run(Some(initial_checked_state.get_untracked()));
        })));

    Effect::new({
        let handle_reset = handle_reset.clone();
        move |_| {
            if let Some(form) = button_ref
                .get()
                .and_then(|button| button.closest("form").ok())
                .flatten()
            {
                let _ = form.add_event_listener_with_callback(
                    "reset",
                    (**handle_reset).as_ref().unchecked_ref(),
                );
            }
        }
    });

    on_cleanup(move || {
        if let Some(form) = button_ref
            .get()
            .and_then(|button| button.closest("form").ok())
            .flatten()
        {
            let _ = form.remove_event_listener_with_callback(
                "reset",
                (**handle_reset).as_ref().unchecked_ref(),
            );
        }
    });

    let context_value = CheckboxContextValue {
        state: checked,
        disabled,
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_refs
                attr:r#type="button"
                attr:role="checkbox"
                attr:aria-checked=move || match checked.get() {
                    CheckedState::Indeterminate => "mixed".to_string(),
                    CheckedState::True => "true".to_string(),
                    CheckedState::False => "false".to_string(),
                }
                attr:aria-required=move || required.get().to_string()
                attr:data-state=move || get_state(checked.get())
                attr:data-disabled=move || disabled.get().then_some("")
                attr:disabled=move || disabled.get().then_some("")
                attr:value=move || value.get()
                on:keydown=compose_callbacks(on_keydown, Some(Callback::new(move |event: KeyboardEvent| {
                    if event.key() == "Enter" {
                        event.prevent_default();
                    }
                })), None)
                on:click=compose_callbacks(on_click, Some(Callback::new(move |event: MouseEvent| {
                    set_checked.run(Some(match checked.get() {
                        CheckedState::False => CheckedState::True,
                        CheckedState::True => CheckedState::False,
                        CheckedState::Indeterminate => CheckedState::True,
                    }));

                    if is_form_control.get() {
                        event.stop_propagation();
                    }
                })), None)
            >
                {children.with_value(|children| children())}
            </Primitive>
            <Show when=move || is_form_control.get()>
                <BubbleInput
                    control_ref=button_ref
                    bubbles=Signal::derive(|| true)
                    name=name
                    value=value
                    checked=checked
                    required=required
                    disabled=disabled
                />
            </Show>
        </Provider>
    }
}

#[component]
pub fn CheckboxIndicator(
    /// Used to force mounting when more control is needed.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let context = expect_context::<CheckboxContextValue>();

    let present = Signal::derive(move || {
        force_mount.get()
            || context.state.get() == CheckedState::Indeterminate
            || context.state.get() == CheckedState::True
    });

    let presence = use_presence(present);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attr:data-state=move || get_state(context.state.get())
                attr:data-disabled=move || context.disabled.get().then_some("")
                attr:style="pointer-events: none;"
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Show>
    }
}

#[component]
fn BubbleInput(
    #[prop(into)] control_ref: AnyNodeRef,
    #[prop(into)] checked: Signal<CheckedState>,
    #[prop(into)] bubbles: Signal<bool>,
    #[prop(into)] required: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] name: Signal<Option<String>>,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = NodeRef::new();
    let prev_checked = use_previous(checked);
    let control_size = use_size(control_ref);

    Effect::new(move |_| {
        if let Some(input) = node_ref.get()
            && prev_checked.get() != checked.get()
        {
            let init = web_sys::EventInit::new();
            init.set_bubbles(bubbles.get());

            let event = web_sys::Event::new_with_event_init_dict("click", &init)
                .expect("Click event should be instantiated.");

            input.set_indeterminate(checked.get() == CheckedState::Indeterminate);
            input.set_checked(matches!(checked.get(), CheckedState::True));

            input
                .dispatch_event(&event)
                .expect("Click event should be dispatched.");
        }
    });

    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            aria-hidden="true"
            checked=move || matches!(checked.get(), CheckedState::True).then_some("")
            required=move || required.get().then_some("")
            disabled=move || disabled.get().then_some("")
            name=move || name.get()
            value=move || value.get()
            tab-index="-1"
            style:transform="translateX(-100%)"
            style:width=move || control_size.get().map(|size| format!("{}px", size.width))
            style:height=move || control_size.get().map(|size| format!("{}px", size.height))
            style:position="absolute"
            style:pointer-events="none"
            style:opacity="0"
            style:margin="0px"
        />
    }
}

fn get_state(checked: CheckedState) -> String {
    (match checked {
        CheckedState::True => "checked",
        CheckedState::False => "unchecked",
        CheckedState::Indeterminate => "indeterminate",
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_state_display() {
        assert_eq!(CheckedState::True.to_string(), "true");
        assert_eq!(CheckedState::False.to_string(), "false");
        assert_eq!(CheckedState::Indeterminate.to_string(), "indeterminate");
    }

    #[test]
    fn get_state_values() {
        assert_eq!(get_state(CheckedState::True), "checked");
        assert_eq!(get_state(CheckedState::False), "unchecked");
        assert_eq!(get_state(CheckedState::Indeterminate), "indeterminate");
    }

    #[test]
    fn checked_state_toggle_logic() {
        // False -> True
        assert_eq!(
            match CheckedState::False {
                CheckedState::False => CheckedState::True,
                CheckedState::True => CheckedState::False,
                CheckedState::Indeterminate => CheckedState::True,
            },
            CheckedState::True
        );
        // True -> False
        assert_eq!(
            match CheckedState::True {
                CheckedState::False => CheckedState::True,
                CheckedState::True => CheckedState::False,
                CheckedState::Indeterminate => CheckedState::True,
            },
            CheckedState::False
        );
        // Indeterminate -> True
        assert_eq!(
            match CheckedState::Indeterminate {
                CheckedState::False => CheckedState::True,
                CheckedState::True => CheckedState::False,
                CheckedState::Indeterminate => CheckedState::True,
            },
            CheckedState::True
        );
    }
}
