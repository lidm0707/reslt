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
        r#"visibility: visible;"#
    } else {
        r#"visibility: hidden;"#
    };

    rsx! {

        div {
            style: format!(
                "{} {}",
                config.modal_base,
                is_visible,
            ),
            div {
                style: config.modal_backdrop,
                div {
                    style: config.modal_container,
                    div {
                        style: r#"margin-top: 0.75rem; text-align: center;"#,
                        h3 {
                            style: config.modal_title,
                            "{use_cx_modal.get_title()}"
                        }
                        div {
                            style: config.modal_content,
                            {use_cx_modal.get_content()}
                        }
                    }
                }
            }
        }
        {children}
    }
}
