use dioxus::prelude::*;

/// Modal size variant
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    FullScreen,
}

impl ModalSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModalSize::Small => "max-w-md",
            ModalSize::Medium => "max-w-lg",
            ModalSize::Large => "max-w-2xl",
            ModalSize::FullScreen => "max-w-full mx-4",
        }
    }
}

/// Modal component with Tailwind CSS
///
/// This is a pure UI component that displays a modal dialog. State management
/// should be handled by your application using reslt_core's UseModal hook.
///
/// # Example
/// ```ignore
/// use reslt_core::prelude::*;
/// // Import from your project structure, e.g.:
/// // use components::modal::*;
///
/// #[component]
/// fn MyComponent() -> Element {
///     let modal = use_modal();
///
///     rsx! {
///         button {
///             onclick: move |_| {
///                 modal.set_title("Confirm Action");
///                 modal.set_content(rsx! {
///                     p { "Are you sure you want to proceed?" }
///                 });
///                 modal.open();
///             },
///             "Open Modal"
///         }
///
///         Modal {
///             is_open: modal.is_open(),
///             title: modal.get_title(),
///             children: modal.get_content(),
///             on_close: move |_| modal.close(),
///         }
///     }
/// }
/// ```
#[component]
pub fn Modal(
    /// Whether the modal is currently open
    #[props(into)]
    is_open: bool,
    /// Modal title text
    #[props(default = String::new())]
    title: String,
    /// Modal content/body
    children: Element,
    /// Callback when the modal should be closed
    on_close: EventHandler<()>,
    /// Modal size variant
    #[props(default = ModalSize::Medium)]
    size: ModalSize,
    /// Whether to show the close button in the header
    #[props(default = true)]
    show_close_button: bool,
    /// Whether clicking the backdrop closes the modal
    #[props(default = true)]
    close_on_backdrop_click: bool,
    /// Whether pressing Escape key closes the modal
    #[props(default = true)]
    close_on_escape: bool,
    /// Additional CSS classes for the modal container
    #[props(default = String::new())]
    class: String,
    /// Optional footer content (action buttons, etc.)
    #[props(default = None)]
    footer: Option<Element>,
) -> Element {
    let size_class = size.as_str();

    rsx! {
        if is_open {
            div {
                class: "fixed inset-0 z-50 overflow-y-auto",
                "aria-labelledby": "modal-title",
                "role": "dialog",
                "aria-modal": "true",

                // Backdrop/Overlay
                div {
                    class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity",
                    onclick: move |_| {
                        if close_on_backdrop_click {
                            on_close.call(());
                        }
                    },
                }

                // Modal container
                div {
                    class: "flex min-h-screen items-center justify-center p-4 text-center sm:p-0",

                    // Modal panel
                    div {
                        class: "relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full {size_class} {class}",

                        // Header
                        if !title.is_empty() || show_close_button {
                            div {
                                class: "flex items-center justify-between border-b border-gray-200 px-4 py-3 sm:px-6",

                                if !title.is_empty() {
                                    h3 {
                                        id: "modal-title",
                                        class: "text-lg font-medium leading-6 text-gray-900",
                                        "{title}"
                                    }
                                }

                                if show_close_button {
                                    button {
                                        r#type: "button",
                                        class: "rounded-md bg-white text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                                        onclick: move |_| on_close.call(()),
                                        "aria-label": "Close",

                                        svg {
                                            class: "h-6 w-6",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "2",
                                                d: "M6 18L18 6M6 6l12 12"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Body
                        div {
                            class: "px-4 py-3 sm:px-6",
                            {children}
                        }

                        // Footer (optional)
                        if let Some(footer_content) = footer {
                            div {
                                class: "bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6",
                                {footer_content}
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Modal header component for custom header content
#[component]
pub fn ModalHeader(
    /// Custom header content
    children: Element,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between border-b border-gray-200 px-4 py-3 sm:px-6 {class}",
            {children}
        }
    }
}

/// Modal body component for custom body content
#[component]
pub fn ModalBody(
    /// Body content
    children: Element,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "px-4 py-3 sm:px-6 {class}",
            {children}
        }
    }
}

/// Modal footer component for action buttons
#[component]
pub fn ModalFooter(
    /// Footer content (typically buttons)
    children: Element,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6 {class}",
            {children}
        }
    }
}

/// Confirm modal with predefined buttons
///
/// # Example
/// ```ignore
/// use reslt_core::prelude::*;
/// // Import from your project structure, e.g.:
/// // use components::modal::*;
///
/// #[component]
/// fn DeleteButton() -> Element {
///     let mut show_confirm = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| show_confirm.set(true),
///             "Delete Item"
///         }
///
///         ConfirmModal {
///             is_open: show_confirm(),
///             title: "Delete Item",
///             message: "Are you sure you want to delete this item? This action cannot be undone.",
///             confirm_text: "Delete",
///             cancel_text: "Cancel",
///             on_confirm: move |_| {
///                 // Perform delete action
///                 show_confirm.set(false);
///             },
///             on_cancel: move |_| show_confirm.set(false),
///         }
///     }
/// }
/// ```
#[component]
pub fn ConfirmModal(
    /// Whether the modal is currently open
    #[props(into)]
    is_open: bool,
    /// Modal title
    #[props(default = String::from("Confirm"))]
    title: String,
    /// Confirmation message
    message: String,
    /// Text for the confirm button
    #[props(default = String::from("Confirm"))]
    confirm_text: String,
    /// Text for the cancel button
    #[props(default = String::from("Cancel"))]
    cancel_text: String,
    /// Button variant for the confirm button (default, danger, warning)
    #[props(default = String::from("default"))]
    confirm_variant: String,
    /// Callback when confirm is clicked
    on_confirm: EventHandler<()>,
    /// Callback when cancel is clicked
    on_cancel: EventHandler<()>,
) -> Element {
    let confirm_button_class = format!(
        "inline-flex w-full justify-center rounded-md border border-transparent px-4 py-2 text-base font-medium text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 sm:w-auto sm:text-sm {}",
        match confirm_variant.as_str() {
            "danger" => "bg-red-600 hover:bg-red-700",
            "warning" => "bg-yellow-600 hover:bg-yellow-700",
            _ => "bg-blue-600 hover:bg-blue-700",
        }
    );

    rsx! {
        Modal {
            is_open,
            title,
            size: ModalSize::Small,
            on_close: move |_| on_cancel.call(()),
            footer: Some(rsx! {
                button {
                    r#type: "button",
                    class: "mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm",
                    onclick: move |_| on_cancel.call(()),
                    "{cancel_text}"
                }
                button {
                    r#type: "button",
                    class: "{confirm_button_class}",
                    onclick: move |_| on_confirm.call(()),
                    "{confirm_text}"
                }
            }),
            p {
                class: "text-sm text-gray-500",
                "{message}"
            }
        }
    }
}

/// Alert modal for displaying information
///
/// # Example
/// ```ignore
/// use reslt_core::modal::*;
/// // Import from your project structure, e.g.:
/// // use components::modal::*;
///
/// fn AlertExample() -> Element {
///     let mut show_alert = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| show_alert.set(true),
///             "Show Alert"
///         }
///
///         AlertModal {
///             is_open: show_alert(),
///             title: "Success",
///             message: "Your changes have been saved successfully!",
///             on_close: move |_| show_alert.set(false),
///         }
///     }
/// }
/// ```
#[component]
pub fn AlertModal(
    /// Whether the modal is currently open
    #[props(into)]
    is_open: bool,
    /// Modal title
    #[props(default = String::new())]
    title: String,
    /// Alert message
    message: String,
    /// Button text
    #[props(default = String::from("OK"))]
    button_text: String,
    /// Callback when close button is clicked
    on_close: EventHandler<()>,
) -> Element {
    rsx! {
        Modal {
            is_open,
            title,
            size: ModalSize::Small,
            on_close: move |_| on_close.call(()),
            footer: Some(rsx! {
                div {
                    class: "bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse",
                    button {
                        r#type: "button",
                        class: "inline-flex w-full justify-center rounded-md border border-transparent bg-blue-600 px-4 py-2 text-base font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 sm:ml-3 sm:w-auto sm:text-sm",
                        onclick: move |_| on_close.call(()),
                        "{button_text}"
                    }
                }
            }),
            p {
                class: "text-sm text-gray-500",
                "{message}"
            }
        }
    }
}

/// Form modal with submit and cancel buttons
///
/// # Example
/// ```ignore
/// use reslt_core::modal::*;
/// // Import from your project structure, e.g.:
/// // use components::modal::*;
/// use dioxus::prelude::*;
///
/// fn FormModalExample() -> Element {
///     let mut show_form = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| show_form.set(true),
///             "Add Item"
///         }
///
///         FormModal {
///             is_open: show_form(),
///             title: "Add New Item",
///             submit_text: "Create",
///             on_submit: move |_| {
///                 // Handle form submission
///                 show_form.set(false);
///             },
///             on_cancel: move |_| show_form.set(false),
///             children: rsx! {
///                 div {
///                     div {
///                         label {
///                             class: "block text-sm font-medium text-gray-700",
///                             "Name"
///                         }
///                         input {
///                             r#type: "text",
///                             class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500",
///                             placeholder: "Enter name"
///                         }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn FormModal(
    /// Whether the modal is currently open
    #[props(into)]
    is_open: bool,
    /// Modal title
    title: String,
    /// Form content
    children: Element,
    /// Text for the submit button
    #[props(default = String::from("Submit"))]
    submit_text: String,
    /// Text for the cancel button
    #[props(default = String::from("Cancel"))]
    cancel_text: String,
    /// Whether to submit button is disabled
    #[props(default = false)]
    submit_disabled: bool,
    /// Callback when form is submitted
    on_submit: EventHandler<()>,
    /// Callback when cancel is clicked
    on_cancel: EventHandler<()>,
    /// Modal size
    #[props(default = ModalSize::Medium)]
    size: ModalSize,
) -> Element {
    rsx! {
        Modal {
            is_open,
            title,
            size,
            on_close: move |_| on_cancel.call(()),
            footer: Some(rsx! {
                button {
                    r#type: "button",
                    class: "mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm",
                    onclick: move |_| on_cancel.call(()),
                    "{cancel_text}"
                }
                button {
                    r#type: "button",
                    disabled: submit_disabled,
                    class: "inline-flex w-full justify-center rounded-md border border-transparent bg-blue-600 px-4 py-2 text-base font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 sm:ml-3 sm:w-auto sm:text-sm disabled:opacity-50 disabled:cursor-not-allowed",
                    onclick: move |_| on_submit.call(()),
                    "{submit_text}"
                }
            }),
            {children}
        }
    }
}

/// Loading modal for displaying loading state
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::modal::*;
///
/// fn LoadingExample() -> Element {
///     let mut is_loading = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| {
///                 is_loading.set(true);
///                 // Simulate async operation
///                 spawn(async move {
///                     tokio::time::sleep(std::time::Duration::from_secs(2)).await;
///                     is_loading.set(false);
///                 });
///             },
///             "Start Loading"
///         }
///
///         LoadingModal {
///             is_open: is_loading(),
///             message: "Please wait while we process your request...",
///         }
///     }
/// }
/// ```
#[component]
pub fn LoadingModal(
    /// Whether the modal is currently open
    #[props(into)]
    is_open: bool,
    /// Loading message
    #[props(default = String::from("Loading..."))]
    message: String,
) -> Element {
    rsx! {
        if is_open {
            div {
                class: "fixed inset-0 z-50 overflow-y-auto",
                "role": "dialog",
                "aria-modal": "true",

                // Backdrop
                div {
                    class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity",
                }

                // Loading content
                div {
                    class: "flex min-h-screen items-center justify-center p-4 text-center sm:p-0",

                    div {
                        class: "relative transform overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6",

                        div {
                            class: "mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-blue-100",

                            svg {
                                class: "h-6 w-6 text-blue-600 animate-spin",
                                fill: "none",
                                view_box: "0 0 24 24",
                                circle {
                                    class: "opacity-25",
                                    cx: "12",
                                    cy: "12",
                                    r: "10",
                                    stroke: "currentColor",
                                    stroke_width: "4",
                                }
                                path {
                                    class: "opacity-75",
                                    fill: "currentColor",
                                    d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                                }
                            }
                        }

                        div {
                            class: "mt-3 text-center sm:mt-5",
                            p {
                                class: "text-sm text-gray-500",
                                "{message}"
                            }
                        }
                    }
                }
            }
        }
    }
}
