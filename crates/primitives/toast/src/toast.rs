use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ToastProviderContextValue {
    toasts: RwSignal<Vec<ToastEntry>>,
    add_toast: Callback<ToastEntry>,
    remove_toast: Callback<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ToastEntry {
    id: String,
    open: RwSignal<bool>,
}

#[component]
pub fn ToastProvider(
    #[prop(into, optional, default = 5000.into())] duration: MaybeProp<i32>,
    #[prop(into, optional)] swipe_direction: MaybeProp<String>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let _duration = Signal::derive(move || duration.get().unwrap_or(5000));
    let _swipe = Signal::derive(move || swipe_direction.get().unwrap_or("right".into()));
    let toasts: RwSignal<Vec<ToastEntry>> = RwSignal::new(vec![]);

    let ctx = ToastProviderContextValue {
        toasts,
        add_toast: Callback::new(move |entry: ToastEntry| {
            toasts.update(|t| t.push(entry));
        }),
        remove_toast: Callback::new(move |id: String| {
            toasts.update(|t| t.retain(|e| e.id != id));
        }),
    };

    view! { <Provider value=ctx>{children.with_value(|c| c())}</Provider> }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ToastContextValue {
    id: String,
    open: RwSignal<bool>,
    on_close: Callback<()>,
}

#[component]
pub fn Toast(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] duration: MaybeProp<i32>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let _duration = Signal::derive(move || duration.get().unwrap_or(5000));
    let open_state = RwSignal::new(open.get().or(default_open.get()).unwrap_or(true));

    Effect::new(move |_| {
        if let Some(o) = open.get() {
            open_state.set(o);
        }
    });

    let id = leptix_core::id::use_id(None).get();

    let toast_ctx = ToastContextValue {
        id: id.clone(),
        open: open_state,
        on_close: Callback::new(move |()| {
            open_state.set(false);
            if let Some(cb) = on_open_change {
                cb.run(false);
            }
        }),
    };

    let present = Signal::derive(move || open_state.get());
    let presence = use_presence(present);

    view! {
        <Provider value=toast_ctx>
            <Show when=move || presence.is_present.get()>
                <Primitive element=html::li as_child=as_child node_ref=node_ref
                    attr:role="status"
                    attr:aria-live="off"
                    attr:aria-atomic="true"
                    attr:data-state=move || if open_state.get() { "open" } else { "closed" }
                    attr:tabindex="0"
                >
                    {children.with_value(|c| c())}
                </Primitive>
            </Show>
        </Provider>
    }
}

#[component]
pub fn ToastTitle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! { <Primitive element=html::div as_child=as_child node_ref=node_ref>{children.with_value(|c| c())}</Primitive> }
}

#[component]
pub fn ToastDescription(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! { <Primitive element=html::div as_child=as_child node_ref=node_ref>{children.with_value(|c| c())}</Primitive> }
}

#[component]
pub fn ToastAction(
    #[prop(into)] alt_text: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            attr:aria-label=alt_text
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ToastClose(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ToastContextValue>();

    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            on:click=move |_| ctx.on_close.run(())
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ToastViewport(
    #[prop(into, optional)] label: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let label = Signal::derive(move || label.get().unwrap_or("Notifications".into()));

    view! {
        <Primitive element=html::ol as_child=as_child node_ref=node_ref
            attr:role="region"
            attr:aria-label=move || label.get()
            attr:tabindex="-1"
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}
