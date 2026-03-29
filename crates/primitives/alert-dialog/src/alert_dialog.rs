use leptos::{context::Provider, prelude::*};
use leptos_node_ref::AnyNodeRef;

// Re-export all Dialog parts with AlertDialog names for complete API
pub use leptix_dialog::Dialog;
pub use leptix_dialog::DialogPortal as AlertDialogPortal;

#[derive(Clone, Debug)]
struct AlertDialogContextValue {
    _cancel_ref: AnyNodeRef,
}

/// Alert Dialog root — wraps Dialog with alertdialog semantics.
/// Unlike Dialog, it does not dismiss on outside click.
#[component]
pub fn AlertDialog(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context_value = AlertDialogContextValue {
        _cancel_ref: AnyNodeRef::new(),
    };

    let on_change = on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}));

    view! {
        <Provider value=context_value>
            <Dialog open=open default_open=default_open on_open_change=on_change modal=true>
                {children.with_value(|children| children())}
            </Dialog>
        </Provider>
    }
}

pub use leptix_dialog::DialogDescription as AlertDialogDescription;
pub use leptix_dialog::DialogOverlay as AlertDialogOverlay;
pub use leptix_dialog::DialogTitle as AlertDialogTitle;
pub use leptix_dialog::DialogTrigger as AlertDialogTrigger;

#[component]
pub fn AlertDialogContent(
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let open_focus = on_open_auto_focus.unwrap_or_else(|| Callback::new(|_: web_sys::Event| {}));
    let close_focus = on_close_auto_focus.unwrap_or_else(|| Callback::new(|_: web_sys::Event| {}));
    let escape =
        on_escape_key_down.unwrap_or_else(|| Callback::new(|_: web_sys::KeyboardEvent| {}));

    view! {
        <leptix_dialog::DialogContent
            on_open_auto_focus=open_focus
            on_close_auto_focus=close_focus
            on_escape_key_down=escape
            on_pointer_down_outside=Callback::new(|event: web_sys::PointerEvent| {
                event.prevent_default();
            })
            on_interact_outside=Callback::new(|event: web_sys::Event| {
                event.prevent_default();
            })
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children())}
        </leptix_dialog::DialogContent>
    }
}

#[component]
pub fn AlertDialogCancel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <leptix_dialog::DialogClose as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </leptix_dialog::DialogClose>
    }
}

#[component]
pub fn AlertDialogAction(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <leptix_dialog::DialogClose as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </leptix_dialog::DialogClose>
    }
}
