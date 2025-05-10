use crate::components::components_toast::toast::ToastContainer;
use crate::components::components_toast::toast_signal::*;
use dioxus::prelude::*;
use dioxus_core::VirtualDom;
#[test]
fn test_toast_creation() {
    // Create a test component that uses the toast
    fn test_component() -> Element {
        let mut toast = use_toast();

        // Test adding different types of toasts
        toast.success("Success message");
        toast.error("Error message");
        toast.warning("Warning message");
        toast.info("Info message");

        // Verify toasts were created correctly
        let toasts = toast.get_toasts();
        assert_eq!(toasts.len(), 4);

        // Verify toast properties
        assert_eq!(toasts[0].message, "Success message");
        assert_eq!(toasts[0].toast_type, ToastType::Success);

        assert_eq!(toasts[1].message, "Error message");
        assert_eq!(toasts[1].toast_type, ToastType::Error);

        assert_eq!(toasts[2].message, "Warning message");
        assert_eq!(toasts[2].toast_type, ToastType::Warning);

        assert_eq!(toasts[3].message, "Info message");
        assert_eq!(toasts[3].toast_type, ToastType::Info);

        // Test removing a toast
        toast.remove(toasts[0].id);
        assert_eq!(toast.get_toasts().len(), 3);

        rsx! {
            div {}
        }
    }

    // Create a virtual DOM with the test component
    let mut dom = VirtualDom::new_with_props(
        |_| {
            rsx! {
                ToastContainer { test_component {} }
            }
        },
        (),
    );

    // Render once to run the test
    dom.rebuild(&mut dioxus_core::NoOpMutations);
}

#[tokio::test]
async fn test_toast_container_rendering() {
    // Create a component that will use the toast container
    #[component]
    fn test_app() -> Element {
        let mut toast = use_context::<Signal<UseToast>>()();

        // Add a toast
        toast.success("Test toast");

        rsx! {}
    }

    // Create a virtual DOM with the ToastContainer wrapping our test component
    let mut vdom = VirtualDom::new_with_props(
        |_| {
            rsx! {
                ToastContainer { test_app {} }
            }
        },
        (),
    );

    vdom.rebuild_in_place();

    let mutation = vdom.render_immediate_to_vec();
    let rendered_str = format!("{:?}", mutation);
    // The rendered output should contain our toast message
    assert!(
        rendered_str.contains("Test toast"),
        "Rendered output should contain toast message"
    );
}
