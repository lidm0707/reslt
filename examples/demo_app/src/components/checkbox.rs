use dioxus::prelude::*;

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
            class: "flex items-center gap-3 p-2 hover:bg-gray-50 rounded transition-colors duration-150 cursor-pointer",

            div {
                class: "relative flex items-center",
                input {
                    r#type: "checkbox",
                    checked: checked ,
                    id: format!("item-{}", index),
                    class: "h-4 w-4 cursor-pointer rounded border-gray-300 text-blue-600 focus:ring-blue-500 focus:ring-2 focus:ring-offset-2",
                    onchange: move |_| {
                        on_toggle.call(item_clone.clone());
                    },
                }
            }

            div {
                class: "flex-1 min-w-0",
                label {
                    r#for: format!("item-{}", index),
                    class: "cursor-pointer select-none text-sm font-medium text-gray-700 hover:text-gray-900",
                    {render_item(item)}
                }
            }
        }
    }
}
