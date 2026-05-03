use dioxus::prelude::*;

/// Form wrapper component
#[component]
pub fn Form(
    #[props(default = "".to_string())] class: String,
    #[props(default = None)] onsubmit: Option<EventHandler<FormEvent>>,
    children: Element,
) -> Element {
    rsx! {
        form {
            class: "space-y-4 {class}",
            onsubmit: move |e: FormEvent| {
                e.prevent_default();
                if let Some(handler) = onsubmit {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}

/// FormField component - wraps label and input together
#[component]
pub fn FormField(
    label: String,
    #[props(default = None)] hint: Option<String>,
    #[props(default = None)] error: Option<String>,
    #[props(default = "block text-sm font-medium text-gray-700".to_string())] label_class: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "space-y-1",
            label {
                class: "{label_class}",
                for: "field-{label}",
                "{label}"
            }
            {children}
            if let Some(hint_text) = hint {
                p {
                    class: "text-xs text-gray-500 mt-1",
                    "{hint_text}"
                }
            }
            if let Some(error_text) = error {
                p {
                    class: "text-xs text-red-600 mt-1",
                    "{error_text}"
                }
            }
        }
    }
}

/// TextInput component
#[component]
pub fn TextInput(
    name: String,
    mut value: Signal<String>,
    #[props(default = "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-3 py-2 border".to_string())]
    class: String,
    #[props(default = "".to_string())] placeholder: String,
    #[props(default = None)] r#type: Option<String>,
    #[props(default = None)] disabled: Option<bool>,
    #[props(default = None)] required: Option<bool>,
) -> Element {
    let input_type = r#type.unwrap_or_else(|| "text".to_string());
    let is_disabled = disabled.unwrap_or(false);
    let is_required = required.unwrap_or(false);
    let mut value_clone = value.clone();

    rsx! {
        input {
            id: "field-{name}",
            name: "{name}",
            r#type: "{input_type}",
            class: "{class}",
            placeholder: "{placeholder}",
            value: "{value_clone()}",
            disabled: "{is_disabled}",
            required: "{is_required}",
            oninput: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            },
            onchange: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            }
        }
    }
}

/// NumberInput component
#[component]
pub fn NumberInput(
    name: String,
    mut value: Signal<String>,
    #[props(default = "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-3 py-2 border".to_string())]
    class: String,
    #[props(default = "".to_string())] placeholder: String,
    #[props(default = None)] min: Option<i64>,
    #[props(default = None)] max: Option<i64>,
    #[props(default = None)] step: Option<String>,
) -> Element {
    let mut value_clone = value.clone();

    rsx! {
        input {
            id: "field-{name}",
            name: "{name}",
            r#type: "number",
            class: "{class}",
            placeholder: "{placeholder}",
            value: "{value_clone()}",
            min: min.unwrap_or(0),
            max: max,
            step: step.unwrap_or_else(|| "1".to_string()),
            oninput: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            },
            onchange: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            }
        }
    }
}

/// EmailInput component
#[component]
pub fn EmailInput(
    name: String,
    mut value: Signal<String>,
    #[props(default = "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-3 py-2 border".to_string())]
    class: String,
    #[props(default = "".to_string())] placeholder: String,
) -> Element {
    let mut value_clone = value.clone();

    rsx! {
        input {
            id: "field-{name}",
            name: "{name}",
            r#type: "email",
            class: "{class}",
            placeholder: "{placeholder}",
            value: "{value_clone()}",
            oninput: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            },
            onchange: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            }
        }
    }
}

/// TextArea component
#[component]
pub fn TextArea(
    name: String,
    mut value: Signal<String>,
    #[props(default = "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-3 py-2 border".to_string())]
    class: String,
    #[props(default = "".to_string())] placeholder: String,
    #[props(default = 3)] rows: u32,
) -> Element {
    let mut value_clone = value.clone();

    rsx! {
        textarea {
            id: "field-{name}",
            name: "{name}",
            class: "{class}",
            placeholder: "{placeholder}",
            rows: "{rows}",
            value: "{value_clone()}",
            oninput: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            },
            onchange: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            }
        }
    }
}

/// Select component
#[component]
pub fn Select(
    name: String,
    mut value: Signal<String>,
    options: Vec<(String, String)>, // (value, label)
    #[props(default = "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-3 py-2 border".to_string())]
    class: String,
    #[props(default = "".to_string())] placeholder: String,
) -> Element {
    let mut value_clone = value.clone();

    rsx! {
        select {
            id: "field-{name}",
            name: "{name}",
            class: "{class}",
            value: "{value_clone()}",
            oninput: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            },
            onchange: move |e: Event<FormData>| {
                *value_clone.write() = e.value();
            },
            if !placeholder.is_empty() {
                option {
                    value: "",
                    "{placeholder}"
                }
            }
            {options.into_iter().map(|(opt_value, opt_label)| {
                let opt_value_clone = opt_value.clone();
                let current_value = value_clone.read().clone();
                let is_selected = current_value == opt_value_clone;

                rsx! {
                    option {
                        value: "{opt_value_clone}",
                        selected: "{is_selected}",
                        "{opt_label}"
                    }
                }
            })}
        }
    }
}

/// Button component
#[component]
pub fn Button(
    #[props(default = "button".to_string())] label: String,
    #[props(default = "button".to_string())] r#type: String,
    #[props(default = "bg-blue-600 hover:bg-blue-700 text-white".to_string())] variant: String,
    #[props(default = "".to_string())] class: String,
    #[props(default = None)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] loading: bool,
    children: Option<Element>,
) -> Element {
    let base_classes = "px-4 py-2 rounded-md text-sm font-medium focus:outline-none focus:ring-2 focus:ring-offset-2 transition-colors disabled:opacity-50 disabled:cursor-not-allowed";

    let variant_classes = match variant.as_str() {
        "primary" => "bg-blue-600 hover:bg-blue-700 text-white focus:ring-blue-500",
        "secondary" => "bg-gray-200 hover:bg-gray-300 text-gray-900 focus:ring-gray-500",
        "danger" => "bg-red-600 hover:bg-red-700 text-white focus:ring-red-500",
        "ghost" => "bg-transparent hover:bg-gray-100 text-gray-700 focus:ring-gray-500",
        "success" => "bg-green-600 hover:bg-green-700 text-white focus:ring-green-500",
        _ => "bg-blue-600 hover:bg-blue-700 text-white focus:ring-blue-500",
    };

    let button_type = r#type.as_str();
    let is_disabled = disabled || loading;

    rsx! {
        button {
            r#type: "{button_type}",
            class: "{base_classes} {variant_classes} {class}",
            onclick: move |e: MouseEvent| {
                if let Some(handler) = onclick {
                    handler.call(e);
                }
            },
            disabled: "{is_disabled}",
            if loading {
                span {
                    class: "inline-block animate-spin mr-2",
                    "⟳"
                }
            }
            if let Some(btn_children) = children {
                {btn_children}
            } else {
                "{label}"
            }
        }
    }
}

/// SubmitButton component - convenience component for form submission
#[component]
pub fn SubmitButton(
    #[props(default = "Submit".to_string())] label: String,
    #[props(default = false)] loading: bool,
    #[props(default = false)] disabled: bool,
) -> Element {
    rsx! {
        Button {
            label: "{label}",
            r#type: "submit".to_string(),
            variant: "primary".to_string(),
            loading,
            disabled
        }
    }
}

/// CancelButton component - convenience component for canceling form actions
#[component]
pub fn CancelButton(
    #[props(default = "Cancel".to_string())] label: String,
    #[props(default = false)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        Button {
            label: "{label}",
            r#type: "button".to_string(),
            variant: "secondary".to_string(),
            onclick,
            disabled
        }
    }
}

/// CheckboxGroup component
#[component]
pub fn CheckboxGroup(
    name: String,
    options: Vec<(String, String, bool)>, // (value, label, checked)
    #[props(default = false)] disabled: bool,
    onchange: EventHandler<Vec<String>>,
) -> Element {
    let mut selected_values = use_signal(|| {
        options
            .iter()
            .filter(|(_, _, checked)| *checked)
            .map(|(value, _, _)| value.clone())
            .collect::<Vec<_>>()
    });

    let onchange_clone = onchange.clone();

    rsx! {
        div {
            class: "space-y-2",
            {options.into_iter().map(|(value, label, default_checked)| {
                let value_clone = value.clone();
                let mut selected_values_clone = selected_values.clone();
                let is_checked = selected_values.read().contains(&value_clone);
                let onchange_inner = onchange_clone.clone();

                rsx! {
                    label {
                        class: "flex items-center space-x-2 cursor-pointer",
                        input {
                            r#type: "checkbox",
                            name: "{name}",
                            value: "{value_clone}",
                            checked: "{is_checked}",
                            disabled: "{disabled}",
                            onchange: move |e: Event<FormData>| {
                                if e.value() == "true" {
                                    if !selected_values_clone.read().contains(&value_clone) {
                                        selected_values_clone.write().push(value_clone.clone());
                                    }
                                } else {
                                    selected_values_clone.write().retain(|v| v != &value_clone);
                                }
                                onchange_inner.call(selected_values_clone.read().clone());
                            }
                        }
                        span {
                            class: "text-sm text-gray-700",
                            "{label}"
                        }
                    }
                }
            })}
        }
    }
}

/// RadioGroup component
#[component]
pub fn RadioGroup(
    name: String,
    mut value: Signal<String>,
    options: Vec<(String, String)>, // (value, label)
    #[props(default = false)] disabled: bool,
) -> Element {
    let mut value_clone = value.clone();

    rsx! {
        div {
            class: "space-y-2",
            {options.into_iter().map(|(opt_value, label)| {
                let opt_value_clone = opt_value.clone();
                let mut value_inner_clone = value_clone.clone();
                let is_selected = value_clone() == opt_value_clone;

                rsx! {
                    label {
                        class: "flex items-center space-x-2 cursor-pointer",
                        input {
                            r#type: "radio",
                            name: "{name}",
                            value: "{opt_value_clone}",
                            checked: "{is_selected}",
                            disabled: "{disabled}",
                            onchange: move |_| {
                                *value_inner_clone.write() = opt_value_clone.clone();
                            }
                        }
                        span {
                            class: "text-sm text-gray-700",
                            "{label}"
                        }
                    }
                }
            })}
        }
    }
}
