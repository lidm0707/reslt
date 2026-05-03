use dioxus::prelude::*;

/// Component that includes the checkbox stylesheet
///
/// # Example
/// ```ignore
/// use dioxus::prelude::*;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         CheckboxStylesheet {}
///         // ... rest of your app
///     }
/// }
/// ```
#[component]
pub fn CheckboxStylesheet() -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!("./checkbox.css"),
        }
    }
}

#[component]
fn CheckboxItem<T>(
    index: usize,
    item: T,
    checked: bool,
    on_toggle: EventHandler<T>,
    render_item: fn(T) -> Element,
) -> Element
where
    T: Clone + PartialEq + Eq + std::fmt::Debug + serde::Serialize + 'static,
{
    let item_clone = item.clone();

    rsx! {
        div {
            class: "reslt-checkbox-group-item",

            div {
                class: "reslt-checkbox-group-item-checkbox",
                input {
                    r#type: "checkbox",
                    checked: checked ,
                    id: format!("item-{}", index),
                    class: "reslt-compact-checkbox",
                    onchange: move |_| {
                        on_toggle.call(item_clone.clone());
                    },
                }
            }

            div {
                class: "reslt-checkbox-group-item-content",
                label {
                    r#for: format!("item-{}", index),
                    class: "reslt-checkbox-group-item-label",
                    {render_item(item)}
                }
            }
        }
    }
}
