use dioxus::prelude::*;

/// Toast notification type
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

/// Toast data structure
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Toast {
    pub id: u32,
    pub message: String,
    pub toast_type: ToastType,
}

/// Individual toast item component with raw CSS
#[component]
pub fn ToastItem(
    /// The toast data to display
    toast: Toast,
    /// Callback when the close button is clicked
    on_close: EventHandler<u32>,
) -> Element {
    let (bg_class, text_class, icon) = match toast.toast_type {
        ToastType::Success => ("reslt-toast-success", "reslt-toast-success-text", "✓"),
        ToastType::Error => ("reslt-toast-error", "reslt-toast-error-text", "✕"),
        ToastType::Warning => ("reslt-toast-warning", "reslt-toast-warning-text", "⚠"),
        ToastType::Info => ("reslt-toast-info", "reslt-toast-info-text", "ℹ"),
    };

    rsx! {
        div {
            class: "reslt-toast-item {bg_class} {text_class}",

            // Icon
            div {
                class: "reslt-toast-icon {bg_class}",
                "{icon}"
            }

            // Message
            div {
                class: "reslt-toast-message",
                "{toast.message}"
            }

            // Close button
            button {
                r#type: "button",
                class: "reslt-toast-close {text_class}",
                "aria-label": "Close",
                onclick: move |_| on_close.call(toast.id),
                "×"
            }
        }
    }
}

/// Container for displaying multiple toast notifications with raw CSS
///
/// # Example
/// ```rust
/// use reslt_core::prelude::*;
///
/// fn MyComponent() -> Element {
///     let toast = use_toast();
///     let toasts = toast.get_toasts();
///
///     rsx! {
///         button {
///             onclick: move |_| toast.success("Operation completed successfully!"),
///             "Show Success Toast"
///         }
///
///         ToastContainer {
///             toasts: toasts.into_iter().map(|t| {
///                 let toast_ref = toast.to_owned();
///                 rsx! {
///                     ToastItem {
///                         key: "{t.id}",
///                         toast: t,
///                         on_close: move |id| toast_ref.remove(id),
///                     }
///                 }
///             }).collect()
///         }
///     }
/// }
/// ```
#[component]
pub fn ToastContainer(
    /// The toast items to display (typically from `reslt_core::use_toast().get_toasts()`)
    #[props(into)]
    toasts: Vec<Element>,
) -> Element {
    rsx! {
        div {
            class: "reslt-toast-container",
            for toast in toasts {
                {toast}
            }
        }
    }
}

/// Convenience function to create a Toast with success type
pub fn success_toast(id: u32, message: String) -> Toast {
    Toast {
        id,
        message,
        toast_type: ToastType::Success,
    }
}

/// Convenience function to create a Toast with error type
pub fn error_toast(id: u32, message: String) -> Toast {
    Toast {
        id,
        message,
        toast_type: ToastType::Error,
    }
}

/// Convenience function to create a Toast with warning type
pub fn warning_toast(id: u32, message: String) -> Toast {
    Toast {
        id,
        message,
        toast_type: ToastType::Warning,
    }
}

/// Convenience function to create a Toast with info type
pub fn info_toast(id: u32, message: String) -> Toast {
    Toast {
        id,
        message,
        toast_type: ToastType::Info,
    }
}
