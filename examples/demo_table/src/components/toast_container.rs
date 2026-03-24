use dioxus::prelude::*;
use reslt_core::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastType {
    fn class(&self) -> &'static str {
        match self {
            ToastType::Success => "bg-green-50 border-green-200 text-green-800",
            ToastType::Error => "bg-red-50 border-red-200 text-red-800",
            ToastType::Warning => "bg-yellow-50 border-yellow-200 text-yellow-800",
            ToastType::Info => "bg-blue-50 border-blue-200 text-blue-800",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ToastType::Success => "✓",
            ToastType::Error => "✕",
            ToastType::Warning => "⚠",
            ToastType::Info => "ℹ",
        }
    }

    fn border_class(&self) -> String {
        self.class()
            .replace("bg-", "border-")
            .split(' ')
            .next()
            .unwrap_or("")
            .to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ToastMessage {
    id: u32,
    message: String,
    toast_type: ToastType,
}

#[component]
pub fn ToastContainer() -> Element {
    let toast_context = use_context::<UseToast>().to_owned();

    // Track toasts in local state
    let mut toasts = use_signal(|| Vec::<ToastMessage>::new());
    let mut next_id = use_signal(|| 0u32);

    // Effect to listen for toast notifications from context
    use_effect(move || {
        // This is a simplified approach - in a real implementation,
        // you'd need a way to subscribe to toast events from the context
        // For now, we'll just return the cleanup
        ()
    });

    rsx! {
        div {
            class: "fixed bottom-4 right-4 z-50 flex flex-col space-y-2",
            {(toasts)().into_iter().map(|toast| {
                // Pre-calculate class values outside of rsx!
                let bg_class = toast.toast_type.class();
                let border_class = toast.toast_type.border_class();
                let icon = toast.toast_type.icon();
                let message = toast.message.clone();
                let id = toast.id;

                rsx! {
                    div {
                        class: "flex items-center w-full max-w-xs p-4 mb-4 text-gray-500 bg-white rounded-lg shadow dark:text-gray-400 dark:bg-gray-800 border-l-4",
                        class: "{border_class}",

                        // Icon
                        div {
                            class: "inline-flex items-center justify-center flex-shrink-0 w-8 h-8 rounded-lg",
                            class: "{bg_class}",
                            "{icon}"
                        }

                        // Message
                        div {
                            class: "ml-3 text-sm font-normal",
                            "{message}"
                        }

                        // Close button
                        button {
                            class: "ml-auto -mx-1.5 -my-1.5 bg-white text-gray-400 hover:text-gray-900 rounded-lg focus:ring-2 focus:ring-gray-300 p-1.5 hover:bg-gray-100 inline-flex h-8 w-8 dark:text-gray-500 dark:hover:text-white dark:bg-gray-800 dark:hover:bg-gray-700",
                            onclick: move |_| {
                                toasts.write().retain(|t| t.id != id);
                            },
                            span {
                                class: "sr-only",
                                "Close"
                            },
                            svg {
                                class: "w-5 h-5",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                "aria-hidden": "true",
                                path {
                                    "fill-rule": "evenodd",
                                    d: "M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z",
                                    "clip-rule": "evenodd"
                                }
                            }
                        }
                    }
                }
            })}
        }
    }
}
