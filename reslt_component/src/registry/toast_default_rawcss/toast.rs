use dioxus::prelude::*;

// Re-export Toast types from reslt_core to ensure type compatibility
pub use reslt_core::prelude::{Toast, ToastType};

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
            class: "reslt-toast-container",
            for toast in toasts {
                {toast}
            }
        }
    }
}
