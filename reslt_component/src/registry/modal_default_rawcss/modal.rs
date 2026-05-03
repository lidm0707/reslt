use dioxus::prelude::*;

/// Modal size variants
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
            ModalSize::Small => "reslt-modal-small",
            ModalSize::Medium => "reslt-modal-medium",
            ModalSize::Large => "reslt-modal-large",
            ModalSize::FullScreen => "reslt-modal-fullscreen",
        }
    }
}

/// Modal component
///
/// A flexible modal dialog component with customizable header, body, and footer.
///
/// # Example
/// ```ignore
/// use reslt_core::prelude::*;
///
/// fn App() -> Element {
///     let mut is_open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| is_open.set(true),
///             "Open Modal"
///         }
///         Modal {
///             is_open: *is_open.read(),
///             title: "My Modal".to_string(),
///             on_close: move |_| is_open.set(false),
///             ModalHeader { "Header Content" }
///             ModalBody { "Body content goes here." }
///             ModalFooter {
///                 button {
///                     onclick: move |_| is_open.set(false),
///                     "Close"
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Modal(
    /// Whether to include the stylesheet (set true once in your app root)
    #[props(default = false)]
    include_stylesheet: bool,
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
        if include_stylesheet {
            ModalStylesheet {}
        }

        if is_open {
            div {
                class: "reslt-modal-backdrop",
                onclick: move |_| {
                    if close_on_backdrop_click {
                        on_close.call(());
                    }
                },
                div {
                    class: "reslt-modal-container {size_class} {class}",
                    onclick: move |e: MouseEvent| {
                        e.stop_propagation();
                    },
                    onkeydown: move |e: Event<KeyboardData>| {
                        if close_on_escape && e.data().key() == Key::Escape {
                            on_close.call(());
                        }
                    },

                    div {
                        class: "reslt-modal-header",
                        if !title.is_empty() {
                            h2 {
                                class: "reslt-modal-title",
                                "{title}"
                            }
                        }
                        if show_close_button {
                            button {
                                class: "reslt-modal-close-button",
                                onclick: move |_| {
                                    on_close.call(());
                                },
                                "×"
                            }
                        }
                    }

                    div {
                        class: "reslt-modal-body",
                        {children}
                    }

                    if let Some(footer_content) = footer {
                        div {
                            class: "reslt-modal-footer",
                            {footer_content}
                        }
                    }
                }
            }
        }
    }
}

/// Modal stylesheet component
///
/// Include this once in your app root to load the necessary CSS.
#[component]
pub fn ModalStylesheet() -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!("./modal.css"),
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
            class: "reslt-modal-header {class}",
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
            class: "reslt-modal-body {class}",
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
            class: "reslt-modal-footer {class}",
            {children}
        }
    }
}

/// Confirm modal with predefined buttons
///
/// # Example
/// ```ignore
/// use reslt_core::prelude::*;
///
/// fn App() -> Element {
///     let mut is_open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| is_open.set(true),
///             "Delete Item"
///         }
///         ConfirmModal {
///             is_open: *is_open.read(),
///             message: "Are you sure you want to delete this item? This action cannot be undone.".to_string(),
///             on_confirm: move |_| {
///                 println!("Confirmed!");
///                 is_open.set(false);
///             },
///             on_close: move |_| is_open.set(false),
///             confirm_variant: "danger".to_string(),
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
    /// Callback when cancel is clicked or modal is closed
    on_close: EventHandler<()>,
    /// Whether to include the stylesheet (set true once in your app root)
    #[props(default = false)]
    include_stylesheet: bool,
) -> Element {
    rsx! {
        Modal {
            include_stylesheet,
            is_open,
            title,
            on_close,
            footer: Some(rsx! {
                div {
                    class: "reslt-modal-actions",
                    button {
                        class: "reslt-button reslt-button-secondary",
                        onclick: move |_| on_close.call(()),
                        "{cancel_text}"
                    }
                    button {
                        class: format_args!("reslt-button reslt-button-{}", confirm_variant),
                        onclick: move |_| {
                            on_confirm.call(());
                        },
                        "{confirm_text}"
                    }
                }
            }),
            ModalBody {
                p {
                    class: "reslt-confirm-message",
                    "{message}"
                }
            }
        }
    }
}

/// Alert modal with a single acknowledge button
///
/// # Example
/// ```ignore
/// use reslt_core::prelude::*;
///
/// fn App() -> Element {
///     let mut is_open = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| is_open.set(true),
///             "Show Alert"
///         }
///         AlertModal {
///             is_open: *is_open.read(),
///             title: "Success".to_string(),
///             message: "Your changes have been saved successfully.".to_string(),
///             on_close: move |_| is_open.set(false),
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
    /// Whether to include the stylesheet (set true once in your app root)
    #[props(default = false)]
    include_stylesheet: bool,
) -> Element {
    rsx! {
        Modal {
            include_stylesheet,
            is_open,
            title,
            on_close,
            footer: Some(rsx! {
                div {
                    class: "reslt-modal-actions",
                    button {
                        class: "reslt-button reslt-button-primary",
                        onclick: move |_| on_close.call(()),
                        "{button_text}"
                    }
                }
            }),
            ModalBody {
                p {
                    class: "reslt-alert-message",
                    "{message}"
                }
            }
        }
    }
}

/// Form modal with submit and cancel buttons
///
/// # Example
/// ```ignore
/// use reslt_core::prelude::*;
///
/// fn App() -> Element {
///     let mut is_open = use_signal(|| false);
///     let mut is_submitting = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| is_open.set(true),
///             "Edit Profile"
///         }
///         FormModal {
///             is_open: *is_open.read(),
///             title: "Edit Profile".to_string(),
///             submit_disabled: *is_submitting.read(),
///             on_submit: move |_| {
///                 is_submitting.set(true);
///                 // Simulate async operation
///                 spawn(async move {
///                     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
///                     is_submitting.set(false);
///                     is_open.set(false);
///                 });
///             },
///             on_close: move |_| is_open.set(false),
///             div {
///                 label { "Name" }
///                 input { placeholder: "Enter name" }
///             }
///             div {
///                 label { "Email" }
///                 input { placeholder: "Enter email" }
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
    /// Callback when cancel is clicked or modal is closed
    on_close: EventHandler<()>,
    /// Whether to include the stylesheet (set true once in your app root)
    #[props(default = false)]
    include_stylesheet: bool,
) -> Element {
    rsx! {
        Modal {
            include_stylesheet,
            is_open,
            title,
            on_close,
            footer: Some(rsx! {
                div {
                    class: "reslt-modal-actions",
                    button {
                        class: "reslt-button reslt-button-secondary",
                        onclick: move |_| on_close.call(()),
                        "{cancel_text}"
                    }
                    button {
                        class: "reslt-button reslt-button-primary",
                        disabled: submit_disabled,
                        onclick: move |_| {
                            if !submit_disabled {
                                on_submit.call(());
                            }
                        },
                        "{submit_text}"
                    }
                }
            }),
            ModalBody {
                form {
                    onsubmit: move |e: FormEvent| {
                        e.prevent_default();
                        on_submit.call(());
                    },
                    {children}
                }
            }
        }
    }
}

/// Loading modal for displaying loading state
///
/// # Example
/// ```ignore
/// use reslt_core::prelude::*;
///
/// fn App() -> Element {
///     let mut is_loading = use_signal(|| false);
///
///     rsx! {
///         button {
///             onclick: move |_| {
///                 is_loading.set(true);
///                 // Simulate loading
///                 spawn(async move {
///                     tokio::time::sleep(std::time::Duration::from_secs(2)).await;
///                     is_loading.set(false);
///                 });
///             },
///             "Start Loading"
///         }
///         LoadingModal {
///             is_open: *is_loading.read(),
///             message: "Processing your request...".to_string(),
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
    /// Whether to include the stylesheet (set true once in your app root)
    #[props(default = false)]
    include_stylesheet: bool,
) -> Element {
    rsx! {
        if is_open {
            div {
                class: "reslt-modal-container reslt-modal-small",
                "role": "dialog",
                "aria-modal": "true",

                if include_stylesheet {
                    ModalStylesheet {}
                }

                div {
                    class: "reslt-loading-modal-content",
                    div {
                        class: "reslt-loading-spinner"
                    }
                    p {
                        class: "reslt-loading-message",
                        "{message}"
                    }
                }
            }
        }
    }
}
