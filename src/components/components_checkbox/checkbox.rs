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
    let is_visible = if visible {
        r#"visibility: visible;"#
    } else {
        r#"visibility: hidden;"#
    };
    println!("is visible {}", visible);

    rsx! {
        div { style: format!(r#"position: absolute; bottom: 8.75rem; {}"#, is_visible),
            div { style: r#"position: fixed; z-index: 40; left: calc(50% - 10rem); width: 20rem; display: flex; flex-direction: column; justify-content: center; align-items: center; background-color: white; padding: 1rem; gap: 1rem; border: 1px solid #e5e7eb; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05); border-radius: 0.5rem;"#,
                {method}
            }
        }
    }
}

// 2 component in 1 fn is not good !
#[component]
pub fn CellCheckBox<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    style: TableConfig,
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
            TableCell { style: style.to_owned().table_cell,
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
    style: TableConfig,
    table: UseTable<T>,
) -> Element {
    rsx! {
        TableHead { style: style.to_owned().table_head,
            {
                let data = table.to_owned().get_rows();
                rsx! {
                    input {
                        r#type: "checkbox",
                        checked: use_context::<UseCheckBox<T>>().is_all_checked(),
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
    let input_style = if disabled {
        format!("{} {}", config.checkbox_input, config.checkbox_disabled)
    } else {
        config.checkbox_input.clone()
    };

    rsx! {
        div { style: config.checkbox_base,
            div { style: config.checkbox_container,
                input {
                    r#type: "checkbox",
                    style: input_style,
                    checked,
                    disabled,
                    onchange: move |evt| { onchange.call(evt.value().parse().unwrap_or(false)) },
                }
                label { style: config.checkbox_label, "{label}" }
            }
        }
    }
}
