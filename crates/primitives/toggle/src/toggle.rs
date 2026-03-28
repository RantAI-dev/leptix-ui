use leptix_core::primitive::{Primitive, compose_callbacks};
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{ev::MouseEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn Toggle(
    /// The controlled state of the toggle.
    #[prop(into, optional)]
    pressed: MaybeProp<bool>,
    /// The state of the toggle when initially rendered. Use `default_pressed` if you do not need to control the state of the toggle. Defaults to `false`.
    #[prop(into, optional)]
    default_pressed: MaybeProp<bool>,
    /// The callback that fires when the state of the toggle changes.
    #[prop(into, optional)]
    on_pressed_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let (pressed, set_pressed) = use_controllable_state(UseControllableStateParams {
        prop: pressed,
        on_change: on_pressed_change.map(|on_pressed_change| {
            Callback::new(move |value| {
                if let Some(value) = value {
                    on_pressed_change.run(value);
                }
            })
        }),
        default_prop: default_pressed,
    });
    let pressed = Signal::derive(move || pressed.get().unwrap_or(false));

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:aria-pressed=move || pressed.get().to_string()
            attr:data-state=move || match pressed.get() {
                true => "on",
                false => "off",
            }
            attr:data-disabled=move || disabled.get().then_some("")
            attr:disabled=move || disabled.get().then_some("")
            on:click=compose_callbacks(on_click, Some(Callback::new(move |_| {
                if !disabled.get() {
                    set_pressed.run(Some(!pressed.get()));
                }
            })), None)
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
