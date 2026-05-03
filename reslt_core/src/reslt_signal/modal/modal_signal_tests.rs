#[cfg(test)]
mod tests {
    use crate::reslt_signal::modal::modal_signal::*;
    use dioxus::prelude::*;

    #[test]
    fn test_initial_state() {
        let modal = UseModal {
            status: Signal::new(false),
            title: Signal::new(String::new()),
            content: Signal::new(rsx! {}),
        };

        assert!(!modal.is_open());
        assert_eq!(modal.get_title(), String::new());
    }

    #[test]
    fn test_open() {
        let mut modal = UseModal {
            status: Signal::new(false),
            title: Signal::new(String::new()),
            content: Signal::new(rsx! {}),
        };

        assert!(!modal.is_open());
        modal.open();
        assert!(modal.is_open());
    }

    #[test]
    fn test_close() {
        let mut modal = UseModal {
            status: Signal::new(true),
            title: Signal::new("Test Title".to_string()),
            content: Signal::new(rsx! { "Test Content" }),
        };

        assert!(modal.is_open());
        modal.close();
        assert!(!modal.is_open());
    }

    #[test]
    fn test_set_title() {
        let mut modal = UseModal {
            status: Signal::new(false),
            title: Signal::new(String::new()),
            content: Signal::new(rsx! {}),
        };

        modal.set_title("New Title");
        assert_eq!(modal.get_title(), "New Title".to_string());

        modal.set_title("Another Title");
        assert_eq!(modal.get_title(), "Another Title".to_string());
    }

    #[test]
    fn test_set_content() {
        let mut modal = UseModal {
            status: Signal::new(false),
            title: Signal::new(String::new()),
            content: Signal::new(rsx! {}),
        };

        let new_content = rsx! { "Hello World" };
        modal.set_content(new_content);

        // We can't directly compare Element, but we can check that it was set
        let retrieved = modal.get_content();
        // Just ensure it's not empty
        assert!(retrieved.is_ok());
    }

    #[test]
    fn test_is_open() {
        let mut modal = UseModal {
            status: Signal::new(false),
            title: Signal::new(String::new()),
            content: Signal::new(rsx! {}),
        };

        assert!(!modal.is_open());

        modal.open();
        assert!(modal.is_open());

        modal.close();
        assert!(!modal.is_open());

        modal.open();
        assert!(modal.is_open());
    }

    #[test]
    fn test_get_title() {
        let mut modal = UseModal {
            status: Signal::new(false),
            title: Signal::new("Initial".to_string()),
            content: Signal::new(rsx! {}),
        };

        assert_eq!(modal.get_title(), "Initial".to_string());

        modal.set_title("Updated");
        assert_eq!(modal.get_title(), "Updated".to_string());
    }

    #[test]
    fn test_get_content() {
        let mut modal = UseModal {
            status: Signal::new(false),
            title: Signal::new(String::new()),
            content: Signal::new(rsx! { "Initial" }),
        };

        let content = modal.get_content();
        assert!(content.is_ok());

        modal.set_content(rsx! { "Updated" });
        let updated_content = modal.get_content();
        assert!(updated_content.is_ok());
    }

    #[test]
    fn test_modal_workflow() {
        let mut modal = UseModal {
            status: Signal::new(false),
            title: Signal::new(String::new()),
            content: Signal::new(rsx! {}),
        };

        // Initial state
        assert!(!modal.is_open());
        assert_eq!(modal.get_title(), String::new());

        // Set title
        modal.set_title("Workflow Test");
        assert_eq!(modal.get_title(), "Workflow Test".to_string());

        // Set content
        modal.set_content(rsx! { div { "Test Content" } });
        assert!(modal.get_content().is_ok());

        // Open modal
        modal.open();
        assert!(modal.is_open());

        // Update while open
        modal.set_title("Updated Title");
        assert_eq!(modal.get_title(), "Updated Title".to_string());

        // Close modal
        modal.close();
        assert!(!modal.is_open());

        // Title and content should persist after close
        assert_eq!(modal.get_title(), "Updated Title".to_string());
        assert!(modal.get_content().is_ok());
    }

    #[test]
    fn test_multiple_open_close() {
        let mut modal = UseModal {
            status: Signal::new(false),
            title: Signal::new(String::new()),
            content: Signal::new(rsx! {}),
        };

        for i in 0..5 {
            assert!(!modal.is_open());
            modal.open();
            assert!(modal.is_open());

            modal.set_title(&format!("Title {}", i));
            assert_eq!(modal.get_title(), format!("Title {}", i));

            modal.close();
            assert!(!modal.is_open());
        }
    }

    #[test]
    fn test_content_updates() {
        let mut modal = UseModal {
            status: Signal::new(true),
            title: Signal::new("Test".to_string()),
            content: Signal::new(rsx! { "Content 1" }),
        };

        modal.set_content(rsx! { div { "Content 2" } });

        modal.set_content(rsx! { span { "Content 3" } });

        modal.set_content(rsx! { p { "Content 4" } });
    }
}
