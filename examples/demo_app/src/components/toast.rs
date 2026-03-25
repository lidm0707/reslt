use dioxus::prelude::*;

// Re-export Toast types from reslt_core for convenience
pub use reslt_core::prelude::{Toast, ToastType};

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
///     let toasts = toast.read().get_toasts();
///
///     rsx! {
///         button {
///             onclick: move |_| toast.write().success("Operation completed successfully!"),
///             "Show Success Toast"
///         }
///
///         ToastContainer {
///             toasts: toasts.into_iter().map(|t| {
///                 let toast_ref = toast.clone();
///                 rsx! {
///                     ToastItem {
///                         key: "{t.id}",
///                         toast: t,
///                         on_close: move |id| toast_ref.write().remove(id),
///                     }
///                 }
///             }).collect()
///         }
///     }
/// }
/// ```
#[component]
pub fn ToastContainer(
    /// The toast items to display (typically from `reslt_core::use_toast().read().get_toasts()`)
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
