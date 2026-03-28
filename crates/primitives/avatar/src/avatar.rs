use leptix_core::primitive::{Primitive, VoidPrimitive};
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use web_sys::{
    HtmlImageElement,
    wasm_bindgen::{JsCast, closure::Closure},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageLoadingStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}

#[derive(Clone, Debug)]
struct AvatarContextValue {
    image_loading_status: ReadSignal<ImageLoadingStatus>,
    on_image_loading_status_change: Callback<ImageLoadingStatus>,
}

#[component]
pub fn Avatar(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let (image_loading_status, set_image_loading_status) = signal(ImageLoadingStatus::Idle);

    let context_value = AvatarContextValue {
        image_loading_status,
        on_image_loading_status_change: Callback::new(move |status| {
            set_image_loading_status.set(status)
        }),
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(into, optional)] src: MaybeProp<String>,
    #[prop(into, optional)] on_loading_status_change: Option<Callback<ImageLoadingStatus>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<AvatarContextValue>();
    let image_loading_status = use_image_loading_status(src);
    let handle_loading_status_change = move |status: ImageLoadingStatus| {
        if let Some(on_loading_status_change) = on_loading_status_change {
            on_loading_status_change.run(status);
        }
        context.on_image_loading_status_change.run(status);
    };

    Effect::new(move |_| {
        let image_loading_status = image_loading_status.get();
        if image_loading_status != ImageLoadingStatus::Idle {
            handle_loading_status_change(image_loading_status);
        }
    });

    view! {
        <Show when=move || image_loading_status.get() == ImageLoadingStatus::Loaded>
            <VoidPrimitive
                element=html::img
                as_child=as_child
                node_ref=node_ref
                attr:src=move || src.get()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </VoidPrimitive>
        </Show>
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(into, optional)] delay_ms: MaybeProp<i32>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<AvatarContextValue>();
    let (can_render, set_can_render) = signal(delay_ms.get().is_none());

    let handler: Closure<dyn Fn()> = Closure::new(move || {
        set_can_render.set(true);
    });

    let timer_id = StoredValue::new(None::<i32>);
    Effect::new(move |_| {
        let window = web_sys::window().expect("Window should exist.");

        if let Some(id) = timer_id.get_value() {
            window.clear_timeout_with_handle(id);
        }

        if let Some(delay_ms) = delay_ms.get() {
            timer_id.set_value(Some(
                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        handler.as_ref().unchecked_ref(),
                        delay_ms,
                    )
                    .expect("Timeout should be set."),
            ));
        }
    });

    on_cleanup(move || {
        if let Some(id) = timer_id.get_value()
            && let Some(window) = web_sys::window()
        {
            window.clear_timeout_with_handle(id);
        }
    });

    view! {
        <Show when=move || can_render.get() && context.image_loading_status.get() != ImageLoadingStatus::Loaded>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Show>
    }
}

fn use_image_loading_status(src: MaybeProp<String>) -> ReadSignal<ImageLoadingStatus> {
    let (loading_status, set_loading_status) = signal(ImageLoadingStatus::Idle);
    let is_mounted = StoredValue::new(true);

    let update_status_loaded: Closure<dyn Fn()> = Closure::new(move || {
        if is_mounted.get_value() {
            set_loading_status.set(ImageLoadingStatus::Loaded);
        }
    });
    let update_status_error: Closure<dyn Fn()> = Closure::new(move || {
        if is_mounted.get_value() {
            set_loading_status.set(ImageLoadingStatus::Error);
        }
    });

    Effect::new(move |_| {
        if let Some(src) = src.get() {
            let document = web_sys::window()
                .expect("Window should exist.")
                .document()
                .expect("Document should exist.");

            let image = document
                .create_element("img")
                .map(|element| element.unchecked_into::<HtmlImageElement>())
                .expect("Image element should be created.");

            set_loading_status.set(ImageLoadingStatus::Loading);

            image
                .add_event_listener_with_callback(
                    "load",
                    update_status_loaded.as_ref().unchecked_ref(),
                )
                .expect("Load event listener should be added.");
            image
                .add_event_listener_with_callback(
                    "error",
                    update_status_error.as_ref().unchecked_ref(),
                )
                .expect("Error event listener should be added.");
            image.set_src(&src);
        } else {
            set_loading_status.set(ImageLoadingStatus::Error);
        }
    });

    on_cleanup(move || {
        is_mounted.set_value(false);
    });

    loading_status
}
