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

/// Individual toast item component with Tailwind CSS
#[component]
pub fn ToastItem(
    /// The toast data to display
    toast: Toast,
    /// Callback when the close button is clicked
    on_close: EventHandler<u32>,
) -> Element {
    let (bg_class, text_class, icon) = match toast.toast_type {
        ToastType::Success => ("bg-green-100", "text-green-800", "✓"),
        ToastType::Error => ("bg-red-100", "text-red-800", "✕"),
        ToastType::Warning => ("bg-yellow-100", "text-yellow-800", "⚠"),
        ToastType::Info => ("bg-blue-100", "text-blue-800", "ℹ"),
    };

    rsx! {
        div {
            class: "flex items-center p-4 mb-4 rounded-lg shadow-md {bg_class} {text_class}",

            // Icon
            div {
                class: "inline-flex items-center justify-center flex-shrink-0 w-8 h-8 mr-3 rounded-lg {bg_class}",
                "{icon}"
            }

            // Message
            div {
                class: "text-sm font-normal",
                "{toast.message}"
            }

            // Close button
            button {
                r#type: "button",
                class: "ml-auto -mx-1.5 -my-1.5 rounded-lg focus:ring-2 p-1.5 inline-flex h-8 w-8 hover:bg-gray-200 {text_class}",
                "aria-label": "Close",
                onclick: move |_| on_close.call(toast.id),
                "×"
            }
        }
    }
}

/// Container for displaying multiple toast notifications with Tailwind CSS
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
            class: "fixed top-0 right-0 p-4 z-50 space-y-4 w-80 max-h-screen overflow-y-auto",
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
