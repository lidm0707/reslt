#![allow(non_snake_case)]
use crate::components::components_modal::modal_signal::use_modal;
use dioxus::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Modal, ModalConfig};

    #[test]
    fn test_modal_initial_state() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let modal_signal = use_modal();
                use_context_provider(|| modal_signal.clone());
                assert!(!modal_signal.is_open(), "Modal should be initially closed.");
                rsx! {
                    Modal { config: ModalConfig::default(),
                        p { "Test" }
                    }
                }
            },
            (),
        );

        vdom.rebuild_in_place();
    }

    #[test]
    fn test_modal_opens_and_closes() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let mut modal_signal = use_modal();
                assert!(!modal_signal.is_open());
                modal_signal.open();
                assert!(
                    modal_signal.is_open(),
                    "Modal should be open after calling open()."
                );

                modal_signal.close();
                assert!(
                    !modal_signal.is_open(),
                    "Modal should be closed after calling close()."
                );

                rsx! {
                    Modal {
                        p { "Test modal open/close" }
                    }
                }
            },
            (),
        );

        vdom.rebuild_in_place();
    }

    #[test]
    fn test_modal_title_content() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let mut modal_signal = use_modal();
                let test_title = "My Test Title";
                let test_content = "This is the test content.";

                modal_signal.set_title(test_title);
                modal_signal.set_content(rsx! {
                    p { "{test_content}" }
                });
                modal_signal.open();
                assert_eq!(
                    modal_signal.get_title(),
                    test_title,
                    "Modal title should be set."
                );
                assert_eq!(
                    modal_signal.get_content(),
                    rsx!{{test_content}},
                    "Modal content should be set."
                );

                rsx! { // Placeholder for rendering
                    Modal {
                        config: ModalConfig::default(),
                        // The actual title and content are rendered by the Modal component
                        // based on the signal values.
                        p { "Test modal title/content" }
                    }
                }
            },
            (),
        );
        vdom.rebuild_in_place();
    }
}
