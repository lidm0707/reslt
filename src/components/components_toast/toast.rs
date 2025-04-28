use std::time::Duration;

use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn ToastContainer(children: Element) -> Element {
    use_context_provider(|| {
        Signal::new(UseToast {
            toasts: Signal::new(Vec::<Toast>::new()),
            counter: Signal::new(0),
        })
    });

    rsx! {
        div {
            class: "fixed top-0 right-0 p-4 z-50 space-y-4 w-80 ",
            style: "max-height: 100vh; overflow-y: auto;",
            {
                use_context::<Signal<UseToast>>()()
                    .get_toasts()
                    .into_iter()
                    .map(|t| {
                        let id = t.id.to_owned();
                        rsx! {
                            ToastItem {
                                key: "{id}",
                                toast: t,
                                on_close: move |_| { use_context::<Signal<UseToast>>()().remove(id) },
                            }
                        }
                    })
            }
        }
        {children}
    }
}

#[component]
fn ToastItem(toast: Toast, on_close: EventHandler<u32>) -> Element {
    let (bg_color, icon) = match toast.toast_type {
        ToastType::Success => ("bg-green-500/50", "✓"),
        ToastType::Error => ("bg-red-500/50", "✕"),
        ToastType::Warning => ("bg-yellow-500/50", "⚠"),
        ToastType::Info => ("bg-blue-500/50", "ℹ"),
    };
    spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        use_context::<Signal<UseToast>>()().to_owned().remove(toast.id);
    });

    rsx! {
        div {
            class: "flex items-center w-full max-w-xs p-4 text-gray-500 bg-white rounded-lg shadow dark:text-gray-400 dark:bg-gray-800",
            role: "alert",
            div { class: "inline-flex items-center justify-center flex-shrink-0 w-8 h-8 {bg_color} rounded-lg text-white",
                span { class: "text-xl font-semibold", "{icon}" }
            }
            div { class: "ml-3 text-sm font-normal", "{toast.message}" }
            button {
                r#type: "button",
                class: "ml-auto -mx-1.5 -my-1.5 bg-white text-gray-400 hover:text-gray-900 rounded-lg focus:ring-2 focus:ring-gray-300 p-1.5 hover:bg-gray-100 inline-flex items-center justify-center h-8 w-8 dark:text-gray-500 dark:hover:text-white dark:bg-gray-800 dark:hover:bg-gray-700",
                onclick: move |_| on_close.call(toast.id),
                span { class: "sr-only", "Close" }
                svg {
                    class: "w-3 h-3",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 14 14",
                    path {
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6",
                    }
                }
            }
        }
    }
}
