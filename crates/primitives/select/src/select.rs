use leptix_core::compose_refs::use_composed_refs;
use leptix_core::dismissable_layer::use_dismissable_layer;
use leptix_core::focus_scope::use_focus_scope;
use leptix_core::id::use_id;
use leptix_core::presence::use_presence;
use leptix_core::primitive::{Primitive, VoidPrimitive};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct SelectContextValue {
    open: RwSignal<bool>,
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
    trigger_ref: AnyNodeRef,
    content_id: String,
    disabled: Signal<bool>,
    name: Signal<Option<String>>,
    required: Signal<bool>,
    form: Signal<Option<String>>,
}

#[component]
pub fn Select(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let name = Signal::derive(move || name.get());
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let form = Signal::derive(move || form.get());
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let open_state = RwSignal::new(open.get().or(Some(false)).unwrap_or(false));

    Effect::new(move |_| {
        if let Some(o) = open.get() {
            open_state.set(o);
        }
    });
    Effect::new(move |_| {
        if let Some(cb) = on_open_change {
            cb.run(open_state.get());
        }
    });

    let (val, set_val) = leptix_core::use_controllable_state::use_controllable_state(
        leptix_core::use_controllable_state::UseControllableStateParams {
            prop: value,
            on_change: on_value_change.map(|cb| {
                Callback::new(move |v: Option<String>| {
                    if let Some(v) = v {
                        cb.run(v);
                    }
                })
            }),
            default_prop: default_value,
        },
    );
    let val = Signal::derive(move || val.get());
    let base_id = use_id(None).get();

    let ctx = SelectContextValue {
        open: open_state,
        value: val,
        on_value_change: Callback::new(move |v: String| {
            set_val.run(Some(v));
            open_state.set(false);
        }),
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-content", base_id),
        disabled,
        name,
        required,
        form,
    };

    view! {
        <Provider value=ctx.clone()>
            {children.with_value(|c| c())}
            <Show when=move || ctx.name.get().is_some()>
                <input
                    type="hidden"
                    name=move || ctx.name.get().unwrap_or_default()
                    value=move || ctx.value.get().unwrap_or_default()
                    required=move || ctx.required.get()
                    form=move || ctx.form.get().unwrap_or_default()
                />
            </Show>
        </Provider>
    }
}

#[component]
pub fn SelectTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<SelectContextValue>();
    let refs = use_composed_refs(vec![node_ref, ctx.trigger_ref]);

    view! {
        <Primitive element=html::button as_child=as_child node_ref=refs
            attr:r#type="button"
            attr:role="combobox"
            attr:aria-expanded=move || ctx.open.get().to_string()
            attr:aria-controls=move || ctx.open.get().then(|| ctx.content_id.clone())
            attr:disabled=move || ctx.disabled.get().then_some("")
            attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
            attr:data-disabled=move || ctx.disabled.get().then_some("")
            attr:data-placeholder=move || ctx.value.get().is_none().then_some("")
            on:click=move |_| { if !ctx.disabled.get() { ctx.open.set(!ctx.open.get()); } }
            on:keydown=move |event: KeyboardEvent| {
                if matches!(event.key().as_str(), "ArrowDown" | "ArrowUp" | "Enter" | " ") && !ctx.disabled.get() {
                    event.prevent_default();
                    ctx.open.set(true);
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn SelectValue(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ctx = expect_context::<SelectContextValue>();
    let placeholder = Signal::derive(move || placeholder.get());

    view! {
        <Primitive element=html::span as_child=as_child node_ref=node_ref>
            {move || ctx.value.get().or_else(|| placeholder.get()).unwrap_or_default()}
        </Primitive>
    }
}

#[component]
pub fn SelectPortal(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<SelectContextValue>();
    view! { <Show when=move || ctx.open.get()>{children.with_value(|c| c())}</Show> }
}

#[component]
pub fn SelectContent(
    /// Which side to position on: "top" | "right" | "bottom" | "left"
    #[prop(into, optional)]
    side: MaybeProp<String>,
    /// Offset from the trigger (pixels).
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// Alignment along the side: "start" | "center" | "end"
    #[prop(into, optional)]
    align: MaybeProp<String>,
    /// Offset along the alignment axis (pixels).
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// Whether to flip/shift to avoid viewport collisions.
    #[prop(into, optional)]
    avoid_collisions: MaybeProp<bool>,
    /// Padding from viewport edge when avoiding collisions (pixels).
    #[prop(into, optional)]
    collision_padding: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    // Reserved for future floating-ui integration
    let _side = side;
    let _side_offset = side_offset;
    let _align = align;
    let _align_offset = align_offset;
    let _avoid_collisions = avoid_collisions;
    let _collision_padding = collision_padding;

    let ctx = expect_context::<SelectContextValue>();
    let present = Signal::derive(move || ctx.open.get());
    let presence = use_presence(present);
    let focus_ref = use_focus_scope(Signal::derive(|| true), Signal::derive(|| true), None, None);
    let dismiss_ref = use_dismissable_layer(
        None,
        None,
        None,
        None,
        Some(Callback::new(move |()| ctx.open.set(false))),
        Signal::derive(move || !ctx.open.get()),
    );
    let refs = use_composed_refs(vec![node_ref, presence.node_ref, focus_ref, dismiss_ref]);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive element=html::div as_child=as_child node_ref=refs
                attr:id=ctx.content_id.clone()
                attr:role="listbox"
                attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
                on:keydown=move |event: KeyboardEvent| {
                    match event.key().as_str() {
                        "ArrowDown" => { event.prevent_default(); focus_select_item(&event, true); }
                        "ArrowUp" => { event.prevent_default(); focus_select_item(&event, false); }
                        _ => {}
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
    }
}

#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<SelectContextValue>();
    let item_value = value.clone();
    let item_value_click = value.clone();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let is_selected =
        Signal::derive(move || ctx.value.get().as_deref() == Some(item_value.as_str()));

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="option"
            attr:aria-selected=move || is_selected.get().to_string()
            attr:data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
            attr:data-disabled=move || disabled.get().then_some("")
            attr:tabindex="-1"
            on:click=move |_| { if !disabled.get() { ctx.on_value_change.run(item_value_click.clone()); } }
            on:keydown=move |event: KeyboardEvent| {
                if (event.key() == "Enter" || event.key() == " ") && !disabled.get() {
                    event.prevent_default();
                    ctx.on_value_change.run(value.clone());
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn SelectSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! { <VoidPrimitive element=html::div as_child=as_child node_ref=node_ref attr:aria-hidden="true">{""}</VoidPrimitive> }
}

// ---------------------------------------------------------------------------
// Icon (decorative trigger icon)
// ---------------------------------------------------------------------------

#[component]
pub fn SelectIcon(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <Primitive element=html::span as_child=as_child node_ref=node_ref attr:aria-hidden="true">
            {children.with_value(|c| c.as_ref().map(|c| c()).unwrap_or_else(|| "▼".into_any()))}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Viewport (scrollable area inside content)
// ---------------------------------------------------------------------------

#[component]
pub fn SelectViewport(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="presentation"
            attr:style="overflow:hidden auto;max-height:var(--leptix-select-content-available-height)"
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Group / Label
// ---------------------------------------------------------------------------

#[component]
pub fn SelectGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let id = use_id(None).get();
    let label_id = format!("{}-label", id);

    view! {
        <Provider value=SelectGroupContextValue { label_id: label_id.clone() }>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="group"
                attr:aria-labelledby=label_id
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[derive(Clone, Debug)]
struct SelectGroupContextValue {
    label_id: String,
}

#[component]
pub fn SelectLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let group_ctx = expect_context::<SelectGroupContextValue>();
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref attr:id=group_ctx.label_id.clone()>
            {children.with_value(|c| c())}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// ItemText / ItemIndicator
// ---------------------------------------------------------------------------

#[component]
pub fn SelectItemText(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::span as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn SelectItemIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    // The indicator is rendered inside a SelectItem. It shows only when selected.
    // For now, consumers wrap it in their own <Show> or use CSS [data-state="checked"].
    view! {
        <Primitive element=html::span as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// ScrollUpButton / ScrollDownButton
// ---------------------------------------------------------------------------

#[component]
pub fn SelectScrollUpButton(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:aria-hidden="true"
            attr:style="flex-shrink:0"
        >
            {children.with_value(|c| c.as_ref().map(|c| c()).unwrap_or_else(|| "▲".into_any()))}
        </Primitive>
    }
}

#[component]
pub fn SelectScrollDownButton(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:aria-hidden="true"
            attr:style="flex-shrink:0"
        >
            {children.with_value(|c| c.as_ref().map(|c| c()).unwrap_or_else(|| "▼".into_any()))}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Arrow
// ---------------------------------------------------------------------------

#[component]
pub fn SelectArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <leptix_core::arrow::Arrow as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </leptix_core::arrow::Arrow>
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn focus_select_item(event: &KeyboardEvent, forward: bool) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };
    let Ok(items) = container.query_selector_all("[role='option']:not([data-disabled])") else {
        return;
    };
    let mut nodes = vec![];
    for i in 0..items.length() {
        if let Some(n) = items.item(i) {
            nodes.push(n);
        }
    }
    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());
    let idx = active.as_ref().and_then(|a| {
        nodes
            .iter()
            .position(|n| n == <web_sys::Element as AsRef<web_sys::Node>>::as_ref(a))
    });
    let next = if forward {
        idx.map(|i| i + 1).filter(|i| *i < nodes.len()).or(Some(0))
    } else {
        idx.and_then(|i| i.checked_sub(1))
            .or(Some(nodes.len().saturating_sub(1)))
    };
    if let Some(i) = next
        && let Some(n) = nodes.get(i)
    {
        use web_sys::wasm_bindgen::JsCast;
        if let Ok(el) = n.clone().dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}
