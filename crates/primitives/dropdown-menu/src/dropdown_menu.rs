use floating_ui_leptos::Padding;
use leptix_core::compose_refs::use_composed_refs;
use leptix_core::dismissable_layer::use_dismissable_layer;
use leptix_core::focus_scope::use_focus_scope;
use leptix_core::id::use_id;
use leptix_core::popper::{
    Popper, PopperAnchor, PopperArrow, PopperContent, parse_align, parse_side,
};
use leptix_core::portal::Portal;
use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

// ---------------------------------------------------------------------------
// Context
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct MenuContextValue {
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    trigger_ref: AnyNodeRef,
    content_id: String,
    dir: Signal<leptix_core::direction::Direction>,
}

#[derive(Clone, Debug)]
struct MenuRadioGroupContextValue {
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
}

#[derive(Clone, Debug)]
struct MenuItemCheckedContextValue {
    checked: Signal<bool>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct SubMenuContextValue {
    open: RwSignal<bool>,
    content_id: String,
    trigger_ref: AnyNodeRef,
}

// ---------------------------------------------------------------------------
// Root
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] dir: MaybeProp<leptix_core::direction::Direction>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let dir = Signal::derive(move || dir.get().unwrap_or(leptix_core::direction::Direction::Ltr));

    let (open_state, set_open) = leptix_core::use_controllable_state::use_controllable_state(
        leptix_core::use_controllable_state::UseControllableStateParams {
            prop: open,
            on_change: on_open_change.map(|cb| {
                Callback::new(move |v: Option<bool>| {
                    if let Some(v) = v {
                        cb.run(v);
                    }
                })
            }),
            default_prop: default_open,
        },
    );
    let open = Signal::derive(move || open_state.get().unwrap_or(false));
    let base_id = use_id(None).get();

    let context = MenuContextValue {
        open,
        on_open_change: Callback::new(move |v: bool| set_open.run(Some(v))),
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-content", base_id),
        dir,
    };

    let context = StoredValue::new(context);
    view! {
        <Popper>
            <Provider value=context.get_value()>
                {children.with_value(|c| c())}
            </Provider>
        </Popper>
    }
}

// ---------------------------------------------------------------------------
// Trigger
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let refs = use_composed_refs(vec![node_ref, ctx.trigger_ref]);
    let content_id = StoredValue::new(ctx.content_id.clone());

    view! {
        <PopperAnchor as_child=true>
            <Primitive element=html::button as_child=as_child node_ref=refs
                attr:r#type="button"
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || ctx.open.get().to_string()
                attr:aria-controls=move || ctx.open.get().then(|| content_id.get_value())
                attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                on:click=move |_| ctx.on_open_change.run(!ctx.open.get())
                on:keydown=move |event: KeyboardEvent| {
                    if matches!(event.key().as_str(), "ArrowDown" | "Enter" | " ") {
                        event.prevent_default();
                        ctx.on_open_change.run(true);
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </PopperAnchor>
    }
}

// ---------------------------------------------------------------------------
// Portal
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(into, optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] _force_mount: MaybeProp<bool>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    view! {
        <Show when=move || ctx.open.get()>
            <Portal container=container container_ref=container_ref>
                {children.with_value(|c| c())}
            </Portal>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Content
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuContent(
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
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

    let popper_side = Signal::derive(move || parse_side(&side.get().unwrap_or("bottom".into())));
    let popper_side_offset = Signal::derive(move || side_offset.get().unwrap_or(0.0));
    let popper_align = Signal::derive(move || parse_align(&align.get().unwrap_or("center".into())));
    let popper_align_offset = Signal::derive(move || align_offset.get().unwrap_or(0.0));
    let popper_avoid_collisions = Signal::derive(move || avoid_collisions.get().unwrap_or(true));
    let popper_collision_padding =
        Signal::derive(move || Padding::All(collision_padding.get().unwrap_or(0.0)));

    let ctx = expect_context::<MenuContextValue>();
    // loop defaults to true; focus_menu_item always wraps (matching Radix default)
    let _do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));
    let present = Signal::derive(move || ctx.open.get());
    let presence = use_presence(present);

    let focus_ref = use_focus_scope(Signal::derive(|| true), Signal::derive(|| true), None, None);
    let dismiss_ref = use_dismissable_layer(
        on_escape_key_down,
        on_pointer_down_outside,
        None,
        None,
        Some(Callback::new(move |()| ctx.on_open_change.run(false))),
        Signal::derive(move || !ctx.open.get()),
    );
    let refs = use_composed_refs(vec![node_ref, presence.node_ref, focus_ref, dismiss_ref]);

    let content_id = StoredValue::new(ctx.content_id.clone());
    let search_buffer: RwSignal<String> = RwSignal::new(String::new());
    let search_timer: RwSignal<Option<i32>> = RwSignal::new(None);

    view! {
        <Show when=move || presence.is_present.get()>
            <PopperContent
                side=popper_side
                side_offset=popper_side_offset
                align=popper_align
                align_offset=popper_align_offset
                avoid_collisions=popper_avoid_collisions
                collision_padding=popper_collision_padding
            >
                <Primitive element=html::div as_child=as_child node_ref=refs
                    attr:id=content_id.get_value()
                    attr:role="menu"
                    attr:aria-orientation="vertical"
                    attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                    attr:tabindex="-1"
                    on:keydown=move |event: KeyboardEvent| {
                        match event.key().as_str() {
                            "Tab" => { event.prevent_default(); }
                            "ArrowDown" | "PageDown" => { event.prevent_default(); focus_menu_item(&event, true); }
                            "ArrowUp" | "PageUp" => { event.prevent_default(); focus_menu_item(&event, false); }
                            "Home" => { event.prevent_default(); focus_menu_item_edge(&event, true); }
                            "End" => { event.prevent_default(); focus_menu_item_edge(&event, false); }
                            key if key.len() == 1 && !event.ctrl_key() && !event.meta_key() => {
                                handle_typeahead(&event, key, search_buffer, search_timer);
                            }
                            _ => {}
                        }
                    }
                >
                    {children.with_value(|c| c())}
                </Primitive>
            </PopperContent>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Group
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref attr:role="group">
            {children.with_value(|c| c())}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Label
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Item
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let handle_select = move || {
        if !disabled.get() {
            if let Some(on_select) = on_select {
                on_select.run(());
            }
            ctx.on_open_change.run(false);
        }
    };

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="menuitem"
            attr:data-disabled=move || disabled.get().then_some("")
            attr:tabindex="-1"
            on:click=move |_| handle_select()
            on:keydown=move |event: KeyboardEvent| {
                if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get() {
                    event.prevent_default();
                    handle_select();
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// CheckboxItem
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let checked = Signal::derive(move || checked.get().unwrap_or(false));

    let item_checked_ctx = MenuItemCheckedContextValue { checked };

    view! {
        <Provider value=item_checked_ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="menuitemcheckbox"
                attr:aria-checked=move || checked.get().to_string()
                attr:data-state=move || if checked.get() { "checked" } else { "unchecked" }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:tabindex="-1"
                on:click=move |_| {
                    if !disabled.get() {
                        if let Some(cb) = on_checked_change { cb.run(!checked.get()); }
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.on_open_change.run(false);
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get() {
                        event.prevent_default();
                        if let Some(cb) = on_checked_change { cb.run(!checked.get()); }
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.on_open_change.run(false);
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

// ---------------------------------------------------------------------------
// RadioGroup + RadioItem
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let value = Signal::derive(move || value.get());
    let radio_ctx = MenuRadioGroupContextValue {
        value,
        on_value_change: Callback::new(move |v: String| {
            if let Some(cb) = on_value_change {
                cb.run(v);
            }
        }),
    };

    view! {
        <Provider value=radio_ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref attr:role="group">
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn DropdownMenuRadioItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let radio_ctx = expect_context::<MenuRadioGroupContextValue>();
    let item_value = value.clone();
    let item_value_click = value.clone();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let checked =
        Signal::derive(move || radio_ctx.value.get().as_deref() == Some(item_value.as_str()));
    let item_checked_ctx = MenuItemCheckedContextValue { checked };

    view! {
        <Provider value=item_checked_ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="menuitemradio"
                attr:aria-checked=move || checked.get().to_string()
                attr:data-state=move || if checked.get() { "checked" } else { "unchecked" }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:tabindex="-1"
                on:click=move |_| {
                    if !disabled.get() {
                        radio_ctx.on_value_change.run(item_value_click.clone());
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.on_open_change.run(false);
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get() {
                        event.prevent_default();
                        radio_ctx.on_value_change.run(value.clone());
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.on_open_change.run(false);
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

// ---------------------------------------------------------------------------
// ItemIndicator
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let checked_ctx = expect_context::<MenuItemCheckedContextValue>();

    view! {
        <Show when=move || force_mount.get() || checked_ctx.checked.get()>
            <Primitive element=html::span as_child=as_child node_ref=node_ref
                attr:data-state=move || if checked_ctx.checked.get() { "checked" } else { "unchecked" }
            >
                {children.with_value(|c| c.as_ref().map(|c| c()))}
            </Primitive>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Separator
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="separator"
            attr:aria-orientation="horizontal"
        >
            {""}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Arrow
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <PopperArrow width=width height=height as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </PopperArrow>
    }
}

// ---------------------------------------------------------------------------
// Sub / SubTrigger / SubContent
// ---------------------------------------------------------------------------

#[component]
pub fn DropdownMenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let open_state = RwSignal::new(open.get().or(default_open.get()).unwrap_or(false));

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

    let base_id = use_id(None).get();
    let sub_ctx = SubMenuContextValue {
        open: open_state,
        content_id: format!("{}-sub", base_id),
        trigger_ref: AnyNodeRef::new(),
    };

    view! {
        <Provider value=sub_ctx>
            <Popper>
                {children.with_value(|c| c())}
            </Popper>
        </Provider>
    }
}

#[component]
pub fn DropdownMenuSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let menu_ctx = expect_context::<MenuContextValue>();
    let sub_ctx = expect_context::<SubMenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let refs = use_composed_refs(vec![node_ref, sub_ctx.trigger_ref]);
    let sub_content_id = StoredValue::new(sub_ctx.content_id.clone());

    view! {
        <PopperAnchor as_child=MaybeProp::derive(|| Some(true))>
            <Primitive element=html::div as_child=as_child node_ref=refs
                attr:role="menuitem"
                attr:aria-haspopup="menu"
                attr:aria-expanded=move || sub_ctx.open.get().to_string()
                attr:aria-controls=move || sub_ctx.open.get().then(|| sub_content_id.get_value())
                attr:data-state=move || if sub_ctx.open.get() { "open" } else { "closed" }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:tabindex="-1"
                on:click=move |_| {
                    if !disabled.get() { sub_ctx.open.set(!sub_ctx.open.get()); }
                }
                on:pointerenter=move |_| {
                    if !disabled.get() { sub_ctx.open.set(true); }
                }
                on:keydown=move |event: KeyboardEvent| {
                    let open_key = if menu_ctx.dir.get() == leptix_core::direction::Direction::Rtl { "ArrowLeft" } else { "ArrowRight" };
                    if event.key() == open_key && !disabled.get() {
                        event.prevent_default();
                        sub_ctx.open.set(true);
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </PopperAnchor>
    }
}

#[component]
pub fn DropdownMenuSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] side_offset: MaybeProp<f64>,
    #[prop(into, optional)] _as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let menu_ctx = expect_context::<MenuContextValue>();
    let sub_ctx = expect_context::<SubMenuContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let present = Signal::derive(move || force_mount.get() || sub_ctx.open.get());
    let presence = use_presence(present);

    // Default sub-menu side: right for LTR, left for RTL
    let popper_side = Signal::derive(move || {
        if menu_ctx.dir.get() == leptix_core::direction::Direction::Rtl {
            parse_side("left")
        } else {
            parse_side("right")
        }
    });
    let popper_side_offset = Signal::derive(move || side_offset.get().unwrap_or(0.0));
    let popper_align = Signal::derive(|| parse_align("start"));
    let popper_align_offset = Signal::derive(|| 0.0);
    let popper_avoid_collisions = Signal::derive(|| true);
    let popper_collision_padding = Signal::derive(|| Padding::All(0.0));

    let dismiss_ref = use_dismissable_layer(
        None,
        None,
        None,
        None,
        Some(Callback::new(move |()| sub_ctx.open.set(false))),
        Signal::derive(move || !sub_ctx.open.get()),
    );
    let refs = use_composed_refs(vec![node_ref, presence.node_ref, dismiss_ref]);

    // Safe triangle: use a grace timer on pointerleave instead of closing immediately.
    // This gives the user time to move diagonally from the sub-trigger to the sub-content.
    let grace_timer: RwSignal<Option<i32>> = RwSignal::new(None);
    let clear_grace = move || {
        if let Some(id) = grace_timer.get_untracked()
            && let Some(w) = web_sys::window()
        {
            w.clear_timeout_with_handle(id);
        }
        grace_timer.set(None);
    };

    on_cleanup(clear_grace);

    view! {
        <Show when=move || presence.is_present.get()>
            <PopperContent
                side=popper_side
                side_offset=popper_side_offset
                align=popper_align
                align_offset=popper_align_offset
                avoid_collisions=popper_avoid_collisions
                collision_padding=popper_collision_padding
                node_ref=refs
                attr:id=sub_ctx.content_id.clone()
                attr:role="menu"
                attr:aria-orientation="vertical"
                attr:data-state=move || if sub_ctx.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
                on:keydown=move |event: KeyboardEvent| {
                    let close_key = if menu_ctx.dir.get() == leptix_core::direction::Direction::Rtl { "ArrowRight" } else { "ArrowLeft" };
                    match event.key().as_str() {
                        "ArrowDown" => { event.prevent_default(); focus_menu_item(&event, true); }
                        "ArrowUp" => { event.prevent_default(); focus_menu_item(&event, false); }
                        k if k == close_key => { event.prevent_default(); sub_ctx.open.set(false); }
                        "Escape" => { sub_ctx.open.set(false); }
                        _ => {}
                    }
                }
                on:pointerenter=move |_| { clear_grace(); }
                on:pointerleave=move |_| {
                    clear_grace();
                    let id = web_sys::window().and_then(|w| {
                        w.set_timeout_with_callback_and_timeout_and_arguments_0(
                            web_sys::wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                                sub_ctx.open.set(false);
                            })
                            .into_js_value()
                            .unchecked_ref(),
                            150, // 150ms grace period for diagonal pointer movement
                        )
                        .ok()
                    });
                    grace_timer.set(id);
                }
            >
                {children.with_value(|c| c())}
            </PopperContent>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn focus_menu_item(event: &KeyboardEvent, forward: bool) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let Ok(items) =
        container.query_selector_all("[role='menuitem']:not([data-disabled]), [role='menuitemcheckbox']:not([data-disabled]), [role='menuitemradio']:not([data-disabled])")
    else {
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
        use web_sys::wasm_bindgen::JsCast;
        let n: &web_sys::Node = a.unchecked_ref();
        nodes.iter().position(|node| node == n)
    });

    let next = if forward {
        idx.map(|i| i + 1).filter(|i| *i < nodes.len()).or(Some(0))
    } else {
        idx.and_then(|i| i.checked_sub(1))
            .or(Some(nodes.len().saturating_sub(1)))
    };

    if let Some(idx) = next
        && let Some(node) = nodes.get(idx)
    {
        use web_sys::wasm_bindgen::JsCast;
        if let Ok(el) = node.clone().dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

/// Focus the first or last menu item.
fn focus_menu_item_edge(event: &KeyboardEvent, first: bool) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let Ok(items) =
        container.query_selector_all("[role='menuitem']:not([data-disabled]), [role='menuitemcheckbox']:not([data-disabled]), [role='menuitemradio']:not([data-disabled])")
    else {
        return;
    };

    let target_idx = if first {
        0
    } else {
        items.length().saturating_sub(1)
    };
    if let Some(node) = items.item(target_idx) {
        use web_sys::wasm_bindgen::JsCast;
        if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

/// Typeahead search: accumulate typed characters and focus matching item.
fn handle_typeahead(
    event: &KeyboardEvent,
    key: &str,
    search_buffer: RwSignal<String>,
    search_timer: RwSignal<Option<i32>>,
) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    // Clear previous timer and start a new 1-second reset
    if let Some(id) = search_timer.get_untracked()
        && let Some(w) = web_sys::window()
    {
        w.clear_timeout_with_handle(id);
    }
    search_buffer.update(|buf| buf.push_str(key));

    let id = web_sys::window().and_then(|w| {
        w.set_timeout_with_callback_and_timeout_and_arguments_0(
            web_sys::wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                search_buffer.set(String::new());
            })
            .into_js_value()
            .unchecked_ref(),
            1000,
        )
        .ok()
    });
    search_timer.set(id);

    let search = search_buffer.get_untracked().to_lowercase();
    let Ok(items) =
        container.query_selector_all("[role='menuitem']:not([data-disabled]), [role='menuitemcheckbox']:not([data-disabled]), [role='menuitemradio']:not([data-disabled])")
    else {
        return;
    };

    // Find the first item whose text starts with the search string
    for i in 0..items.length() {
        if let Some(node) = items.item(i) {
            use web_sys::wasm_bindgen::JsCast;
            if let Some(text) = node.text_content()
                && text.trim().to_lowercase().starts_with(&search)
                && let Ok(el) = node.dyn_into::<web_sys::HtmlElement>()
            {
                let _ = el.focus();
                return;
            }
        }
    }
}
