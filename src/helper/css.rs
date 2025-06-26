use dioxus::prelude::*;
const CSS: &str = include_str!("../../assets/output.css");
pub fn css_helper() -> Element {
    rsx! { document::Stylesheet { href: CSS }}
}
