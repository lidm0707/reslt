use crate::prelude::*;
use dioxus::prelude::*;
use serde::Serialize;
use std::fmt::Debug;
use crate::components::components_checkbox::checkbox_config::CheckboxConfig;

#[component]
pub fn CheckBox<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    checkbox: UseCheckBox<T>,
    #[props(default = CheckboxConfig::default())] config: CheckboxConfig,
    method: Element,
    children: Element,
) -> Element {
    provide_context(checkbox);
    let is_visible = if use_context::<UseCheckBox<T>>().get_checked_data().len() > 0 {
        "visible"
    } else {
        "invisible"
    };
    rsx! {
        div { 
            class: format!("fixed bottom-10 left-[calc(50%-10rem)] w-xs flex flex-col justify-center justify-items-center bg-white p-4 gap-4 border border-gray-200 shadow-lg rounded-lg p-4 w-auto z-40 {}", is_visible),
            {method}
        }
        {children}
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
                    checked: checked,
                    disabled: disabled,
                    onchange: move |evt| {
                        onchange.call(evt.value().parse().unwrap_or(false))
                    }
                }
                label { class: config.checkbox_label, "{label}" }
            }
        }
    }
}
