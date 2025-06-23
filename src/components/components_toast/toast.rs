use async_std::task::sleep;
use std::time::Duration;

use crate::components::components_toast::toast_config::ToastConfig;
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn ToastContainer(
    #[props(default = ToastConfig::default())] config: ToastConfig,
    children: Element,
) -> Element {
    use_context_provider(|| {
        Signal::new(UseToast {
            toasts: Signal::new(Vec::<Toast>::new()),
            counter: Signal::new(0),
        })
    });

    rsx! {
        div {
            class: config.toast_container,
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
                                config: config.clone(),
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
fn ToastItem(toast: Toast, config: ToastConfig, on_close: EventHandler<u32>) -> Element {
    let (toast_class, icon) = match toast.toast_type {
        ToastType::Success => (config.toast_success.clone(), "✓"),
        ToastType::Error => (config.toast_error.clone(), "✕"),
        ToastType::Warning => (config.toast_warning.clone(), "⚠"),
        ToastType::Info => (config.toast_info.clone(), "ℹ"),
    };

    spawn(async move {
        sleep(Duration::from_secs(2)).await;
        use_context::<Signal<UseToast>>()()
            .to_owned()
            .remove(toast.id);
    });

    rsx! {
        div {
            class: format!("{} {}", config.toast_item, toast_class),
            div {
                class: config.toast_icon,
                "{icon}"
            }
            div {
                class: config.toast_message,
                "{toast.message}"
            }
            button {
                r#type: "button",
                class: config.toast_close_button,
                "aria-label": "Close",
                onclick: move |_| on_close.call(toast.id),
                "×"
            }
        }
    }
}
