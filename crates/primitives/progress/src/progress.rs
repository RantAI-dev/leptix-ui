use std::fmt::{Display, Formatter};

use leptix_core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

const DEFAULT_MAX: f64 = 100.0;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProgressState {
    Indeterminate,
    Complete,
    Loading,
}

impl Display for ProgressState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProgressState::Indeterminate => "indeterminate",
                ProgressState::Complete => "complete",
                ProgressState::Loading => "loading",
            }
        )
    }
}

#[derive(Clone, Debug)]
struct ProgressContextValue {
    value: Signal<Option<f64>>,
    max: Signal<f64>,
}

#[component]
pub fn Progress(
    #[prop(into, optional)] value: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] get_value_label: Option<Box<dyn Fn(f64, f64) -> String + Send + Sync>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let get_value_label = get_value_label.unwrap_or(Box::new(default_get_value_label));
    let max = Signal::derive(move || {
        max.get()
            .and_then(|max| match max == 0.0 {
                true => None,
                false => Some(max),
            })
            .unwrap_or(DEFAULT_MAX)
    });
    let value = Signal::derive(move || value.get().map(|value| value.min(max.get()).max(0.0)));

    let value_label =
        Signal::derive(move || value.get().map(|value| get_value_label(value, max.get())));

    let context_value = ProgressContextValue { value, max };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:aria-valuemax=move || max.get().to_string()
                attr:aria-valuemin="0"
                attr:aria-valuenow=move || value.get().map(|v| v.to_string())
                attr:aria-valuetext=move || value_label.get()
                attr:role="progressbar"
                attr:data-state=move || get_progress_state(value.get(), max.get()).to_string()
                attr:data-value=move || value.get().map(|v| v.to_string())
                attr:data-max=move || max.get().to_string()
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ProgressIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ProgressContextValue>();

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:data-state=move || get_progress_state(context.value.get(), context.max.get()).to_string()
            attr:data-value=move || context.value.get().map(|v| v.to_string())
            attr:data-max=move || context.max.get().to_string()
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

fn default_get_value_label(value: f64, max: f64) -> String {
    format!("{}%", (value / max).round() * 100.0)
}

fn get_progress_state(value: Option<f64>, max_value: f64) -> ProgressState {
    match value {
        Some(value) => match value == max_value {
            true => ProgressState::Complete,
            false => ProgressState::Loading,
        },
        None => ProgressState::Indeterminate,
    }
}
