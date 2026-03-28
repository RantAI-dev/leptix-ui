use leptix_core::id::use_id;
use leptix_core::primitive::{Primitive, VoidPrimitive};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputValidationType {
    #[default]
    Numeric,
    Alphanumeric,
    Text,
}

impl InputValidationType {
    fn pattern(&self) -> &'static str {
        match self {
            InputValidationType::Numeric => r"[0-9]",
            InputValidationType::Alphanumeric => r"[a-zA-Z0-9]",
            InputValidationType::Text => r".",
        }
    }

    fn is_valid(&self, ch: char) -> bool {
        match self {
            InputValidationType::Numeric => ch.is_ascii_digit(),
            InputValidationType::Alphanumeric => ch.is_ascii_alphanumeric(),
            InputValidationType::Text => true,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct OtpFieldContextValue {
    value: RwSignal<Vec<String>>,
    max_length: usize,
    validation_type: InputValidationType,
    disabled: Signal<bool>,
    base_id: String,
    on_complete: Option<Callback<String>>,
}

#[component]
pub fn OneTimePasswordField(
    /// Number of input segments.
    #[prop(into, optional, default = 6.into())]
    max_length: MaybeProp<usize>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] on_complete: Option<Callback<String>>,
    #[prop(into, optional)] validation_type: Option<InputValidationType>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let max_length = max_length.get().unwrap_or(6);
    let validation_type = validation_type.unwrap_or_default();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let base_id = use_id(None).get();

    let initial: Vec<String> = default_value
        .get()
        .map(|s| s.chars().map(|c| c.to_string()).collect())
        .unwrap_or_else(|| vec![String::new(); max_length]);

    let value = RwSignal::new(initial);

    Effect::new(move |_| {
        let v = value.get();
        let combined: String = v.iter().map(|s| s.as_str()).collect();
        if let Some(cb) = on_value_change {
            cb.run(combined.clone());
        }
        if combined.len() == max_length
            && v.iter().all(|s| !s.is_empty())
            && let Some(cb) = on_complete
        {
            cb.run(combined);
        }
    });

    let ctx = OtpFieldContextValue {
        value,
        max_length,
        validation_type,
        disabled,
        base_id,
        on_complete,
    };

    view! {
        <Provider value=ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:data-disabled=move || disabled.get().then_some("")
                attr:style="display:flex;gap:4px"
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn OneTimePasswordFieldInput(
    #[prop(into, optional)] index: Option<usize>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ctx = expect_context::<OtpFieldContextValue>();
    let idx = index.unwrap_or(0);

    let input_value = Signal::derive(move || ctx.value.get().get(idx).cloned().unwrap_or_default());
    let base_id = ctx.base_id.clone();
    let base_id_input = ctx.base_id.clone();
    let base_id_keydown = ctx.base_id.clone();

    view! {
        <VoidPrimitive element=html::input as_child=as_child node_ref=node_ref
            attr:id=format!("{}-input-{}", base_id, idx)
            attr:r#type="text"
            attr:inputmode="numeric"
            attr:pattern=ctx.validation_type.pattern()
            attr:maxlength="1"
            attr:autocomplete="one-time-code"
            attr:value=move || input_value.get()
            attr:disabled=move || ctx.disabled.get().then_some("")
            attr:data-leptix-otp-input=""
            attr:aria-label=format!("Digit {}", idx + 1)
            attr:style="width:2.5em;text-align:center"
            on:input=move |event: leptos::ev::Event| {
                use web_sys::wasm_bindgen::JsCast;
                let target = event.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok());
                if let Some(input) = target {
                    let val = input.value();
                    let ch = val.chars().last();
                    if let Some(ch) = ch {
                        if ctx.validation_type.is_valid(ch) {
                            ctx.value.update(|v| {
                                if let Some(slot) = v.get_mut(idx) {
                                    *slot = ch.to_string();
                                }
                            });
                            // Auto-advance to next input
                            if idx + 1 < ctx.max_length
                                && let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                                    let next_id = format!("{}-input-{}", base_id_input, idx + 1);
                                    if let Some(next) = doc.get_element_by_id(&next_id)
                                        && let Ok(el) = next.dyn_into::<web_sys::HtmlElement>() {
                                            let _ = el.focus();
                                        }
                                }
                        } else {
                            input.set_value(&input_value.get_untracked());
                        }
                    }
                }
            }
            on:keydown=move |event: KeyboardEvent| {
                use web_sys::wasm_bindgen::JsCast;
                if event.key() == "Backspace" {
                    let current = ctx.value.get().get(idx).cloned().unwrap_or_default();
                    if current.is_empty() && idx > 0 {
                        // Move to previous input
                        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                            let prev_id = format!("{}-input-{}", base_id_keydown, idx - 1);
                            if let Some(prev) = doc.get_element_by_id(&prev_id)
                                && let Ok(el) = prev.dyn_into::<web_sys::HtmlElement>() {
                                    let _ = el.focus();
                                }
                        }
                    } else {
                        ctx.value.update(|v| {
                            if let Some(slot) = v.get_mut(idx) {
                                slot.clear();
                            }
                        });
                    }
                }
            }
        >
            {""}
        </VoidPrimitive>
    }
}

#[component]
pub fn OneTimePasswordFieldHiddenInput(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ctx = expect_context::<OtpFieldContextValue>();
    let combined = Signal::derive(move || {
        ctx.value
            .get()
            .iter()
            .map(|s| s.as_str())
            .collect::<String>()
    });

    view! {
        <input
            node_ref=node_ref
            type="hidden"
            name=move || name.get().unwrap_or_default()
            value=move || combined.get()
            aria-hidden="true"
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_validation() {
        let v = InputValidationType::Numeric;
        assert!(v.is_valid('5'));
        assert!(v.is_valid('0'));
        assert!(!v.is_valid('a'));
        assert!(!v.is_valid(' '));
    }

    #[test]
    fn alphanumeric_validation() {
        let v = InputValidationType::Alphanumeric;
        assert!(v.is_valid('a'));
        assert!(v.is_valid('Z'));
        assert!(v.is_valid('5'));
        assert!(!v.is_valid(' '));
        assert!(!v.is_valid('-'));
    }

    #[test]
    fn text_validation() {
        let v = InputValidationType::Text;
        assert!(v.is_valid('a'));
        assert!(v.is_valid(' '));
        assert!(v.is_valid('!'));
    }

    #[test]
    fn validation_patterns() {
        assert_eq!(InputValidationType::Numeric.pattern(), r"[0-9]");
        assert_eq!(InputValidationType::Alphanumeric.pattern(), r"[a-zA-Z0-9]");
        assert_eq!(InputValidationType::Text.pattern(), r".");
    }
}
