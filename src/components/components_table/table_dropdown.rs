use dioxus::prelude::*;

#[component]
pub fn ActionDropdown(item: Element) -> Element {
    let dropdown_open = use_context_provider(|| Signal::new(false));
    let isVisible = if dropdown_open() {
        r#"visibility: visible;"#
    } else {
        r#"visibility: hidden;"#
    };

    rsx! {
        div { style: r#"position: relative;"#,
            {
                let mut dropdown_open_clone = dropdown_open.to_owned();
                rsx! {
                    button { style: r#""#, onclick: move |_| dropdown_open_clone.set(!dropdown_open()), "Actions ⌄" }
                }
            }

            div {
                style: format!(
                    r#"position: absolute; z-index: 2; background-color:#202838; border-radius: 0.5rem; box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04); width: 11rem; border: 1px solid #e5e7eb; {}"#,
                    isVisible
                ),
                {item}
            }
        }
    }
}

#[component]
pub fn ItemDropdown(title: String, action: EventHandler<MouseEvent>) -> Element {
    let close = use_context::<Signal<bool>>();

    rsx! {
        div {
            style: r#"padding-left: 1rem; padding-right: 1rem; padding-top: 0.5rem; padding-bottom: 0.5rem;"#,
            onclick: move |evt| {
                action.call(evt);
                close.to_owned().set(false);
            },
            "{title}"
        }
    }
}
