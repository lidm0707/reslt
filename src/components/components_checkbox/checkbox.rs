use crate::components::components_checkbox::checkbox_config::CheckboxConfig;
use dioxus::prelude::*;


#[component]
pub fn CheckBox(
    #[props(default = CheckboxConfig::default())] config: CheckboxConfig,
    visible:bool,
    method: Element,
) -> Element {
    let is_visible = if visible {
        "visible"
    } else {
        "invisible"
    };
    println!("is visible {}", visible);
    
    rsx! {
        div {
            class: format!(
                "absolute bottom-10 z-40 left-[calc(50%-10rem)] w-xs flex flex-col justify-center items-center bg-white p-4 gap-4 border border-gray-200 shadow-lg rounded-lg  {}",
                is_visible,
            ),
            {method}
        }
    }
}

#[component]
pub fn SingleCheckbox(
    #[props(default = false)] checked: bool,
    #[props(default = false)] disabled: bool,
    #[props(default = "")] label: &'static str,
    #[props(default = CheckboxConfig::default())] config: CheckboxConfig,
    onchange: EventHandler<bool>,
) -> Element {
    let input_class = if disabled {
        format!("{} {}", config.checkbox_input, config.checkbox_disabled)
    } else {
        config.checkbox_input.clone()
    };

    rsx! {
        div { class: config.checkbox_base,
            div { class: config.checkbox_container,
                input {
                    r#type: "checkbox",
                    class: input_class,
                    checked,
                    disabled,
                    onchange: move |evt| { onchange.call(evt.value().parse().unwrap_or(false)) },
                }
                label { class: config.checkbox_label, "{label}" }
            }
        }
    }
}
