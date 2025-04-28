use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Modal(class: Option<String>,children:Element) -> Element {
    let use_modal = use_modal();
    use_context_provider(|| use_modal);
    let use_cx_modal = use_context::<UseModal>();
    let is_visible = if use_cx_modal.is_open() {
        "visible"
    } else {
        "invisible"
    };

    rsx! {
        div {
            class: format!(
                "fixed inset-0 bg-gray-600/50  overflow-y-auto h-full w-full z-10 {}",
                is_visible,
            ),
            div { class: "relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white",
                div { class: "mt-3 text-center",

                    h3 { class: "text-lg leading-6 font-medium text-gray-900",
                        "{use_cx_modal.get_title()}"
                    }
                    div { class: "mt-2 px-7 py-3", {use_cx_modal.get_content()} }
                
                }
            }
        }
        {children}
    }
}
