use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Modal(
    #[props(default = ModalConfig::default())] config: ModalConfig,
    children: Element,
) -> Element {
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
                "{} {}",
                config.modal_base,
                is_visible,
            ),
            div {
                class: config.modal_backdrop,
                div {
                    class: config.modal_container,
                    div {
                        class: "mt-3 text-center",
                        h3 {
                            class: config.modal_title,
                            "{use_cx_modal.get_title()}"
                        }
                        div {
                            class: config.modal_content,
                            {use_cx_modal.get_content()}
                        }
                    }
                }
            }
        }
        {children}
    }
}
