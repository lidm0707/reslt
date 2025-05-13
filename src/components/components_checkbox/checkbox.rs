use std::fmt::Debug;

use crate::{
    components::components_checkbox::checkbox_config::CheckboxConfig,
    prelude::{FieldAccessible, TableCell, TableConfig, TableHead, UseCheckBox, UseTable},
};
use dioxus::prelude::*;
use serde::Serialize;

#[component]
pub fn CheckBox(
    #[props(default = CheckboxConfig::default())] config: CheckboxConfig,
    visible: bool,
    method: Element,
) -> Element {
    let is_visible = if visible { "visible" } else { "invisible" };
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

// 2 component in 1 fn is not good !
#[component]
pub fn CellCheckBox<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    class: TableConfig,
    row: T,
) -> Element {
    {
        let row = row.to_owned();
        let mut checked = use_signal(|| false);
        let update_check_all = use_context::<UseCheckBox<T>>().is_all_checked();
        use_effect(use_reactive!(|(update_check_all)| {
            checked.set(update_check_all);
        }));
        rsx! {
            TableCell { class: class.to_owned().table_cell,
                input {
                    r#type: "checkbox",
                    checked: checked(),
                    onchange: move |_| {
                        if !checked() {
                            use_context::<UseCheckBox<T>>().to_owned().push_checked_data(row.to_owned());
                            checked.set(true);
                        } else {
                            use_context::<UseCheckBox<T>>().to_owned().remove(row.to_owned());
                            checked.set(false);
                        }
                    },
                }
            }
        }
    }
}


// 2 component in 1 fn is not good !
#[component]
pub fn HeadCellCheckBox<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    class: TableConfig,
    table: UseTable<T>,
) -> Element {
    rsx! {
        TableHead { class: class.to_owned().table_head,
            {
                let data = table.to_owned().get_rows();
                rsx! {
                    input {
                        r#type: "checkbox",
                        onchange: move |_| {
                            use_context::<UseCheckBox<T>>().set_all_checked(data.to_owned());
                        },
                    }
                }
            }
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
