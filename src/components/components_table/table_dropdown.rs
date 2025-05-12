use dioxus::prelude::*;

#[component]
pub fn ActionDropdown(item: Element) -> Element {
    let dropdown_open = use_context_provider(|| Signal::new(false));
    let isVisible = if dropdown_open() {
        "visible"
    } else {
        "invisible"
    };
    rsx! {
        div { class: "relative ",
            {
                let mut dropdown_open_clone = dropdown_open.to_owned();
                rsx! {
                    button { class: "", onclick: move |_| dropdown_open_clone.set(!dropdown_open()), "Actions ⌄" }
                }
            }

            div {
                class: format!(
                    "absolute z-30 bg-white dark:bg-gray-800  rounded-lg shadow-xl w-44 border border-gray-200 dark:border-gray-700   {isVisible}",
                ),
                {item}
            }
        }
    }
}

#[component]
pub fn ItemDropdown(title: String, action: EventHandler<MouseEvent>) -> Element {
    let close = use_context::<Signal<bool>>();

    rsx! {
        div {
            class: "px-4 py-2 hover:bg-gray-50 dark:hover:bg-gray-600 ",
            onclick: move |evt| {
                action.call(evt);
                close.to_owned().set(false);
            },
            "{title}"
        }
    }
}

