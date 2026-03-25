use dioxus::prelude::*;

/// Container component - provides consistent layout with max-width and centering
#[component]
pub fn Container(
    #[props(default = "py-8".to_string())] padding: String,
    #[props(default = "".to_string())] class: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 {padding} {class}",
            {children}
        }
    }
}
