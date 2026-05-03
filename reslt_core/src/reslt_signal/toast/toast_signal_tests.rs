#[cfg(test)]
mod tests {
    use crate::reslt_signal::toast::toast_signal::*;
    use dioxus::prelude::*;

    fn with_runtime<F: FnOnce() -> R, R>(f: F) -> R {
        let dom = VirtualDom::new(|| rsx! {});
        dom.in_scope(ScopeId::ROOT, f)
    }

    #[test]
    fn test_toast_type_equality() {
        assert_eq!(ToastType::Success, ToastType::Success);
        assert_eq!(ToastType::Error, ToastType::Error);
        assert_eq!(ToastType::Warning, ToastType::Warning);
        assert_eq!(ToastType::Info, ToastType::Info);
    }

    #[test]
    fn test_toast_type_inequality() {
        assert_ne!(ToastType::Success, ToastType::Error);
        assert_ne!(ToastType::Warning, ToastType::Info);
        assert_ne!(ToastType::Success, ToastType::Warning);
    }

    #[test]
    fn test_toast_creation() {
        let toast = Toast {
            id: 1,
            message: "Test message".to_string(),
            toast_type: ToastType::Success,
        };

        assert_eq!(toast.id, 1);
        assert_eq!(toast.message, "Test message");
        assert_eq!(toast.toast_type, ToastType::Success);
    }

    #[test]
    fn test_use_toast_initial_state() {
        with_runtime(|| {
            let use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            assert_eq!(use_toast.get_toasts().len(), 0);
            assert_eq!((*use_toast.counter)(), 0);
        });
    }

    #[test]
    fn test_use_toast_success() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            use_toast.success("Success message");

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 1);
            assert_eq!(toasts[0].id, 1);
            assert_eq!(toasts[0].message, "Success message");
            assert_eq!(toasts[0].toast_type, ToastType::Success);
            assert_eq!((*use_toast.counter)(), 1);
        });
    }

    #[test]
    fn test_use_toast_error() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            use_toast.error("Error message");

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 1);
            assert_eq!(toasts[0].id, 1);
            assert_eq!(toasts[0].message, "Error message");
            assert_eq!(toasts[0].toast_type, ToastType::Error);
        });
    }

    #[test]
    fn test_use_toast_warning() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            use_toast.warning("Warning message");

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 1);
            assert_eq!(toasts[0].id, 1);
            assert_eq!(toasts[0].message, "Warning message");
            assert_eq!(toasts[0].toast_type, ToastType::Warning);
        });
    }

    #[test]
    fn test_use_toast_info() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            use_toast.info("Info message");

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 1);
            assert_eq!(toasts[0].id, 1);
            assert_eq!(toasts[0].message, "Info message");
            assert_eq!(toasts[0].toast_type, ToastType::Info);
        });
    }

    #[test]
    fn test_multiple_toasts() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            use_toast.success("Success");
            use_toast.error("Error");
            use_toast.warning("Warning");
            use_toast.info("Info");

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 4);
            assert_eq!(toasts[0].toast_type, ToastType::Success);
            assert_eq!(toasts[1].toast_type, ToastType::Error);
            assert_eq!(toasts[2].toast_type, ToastType::Warning);
            assert_eq!(toasts[3].toast_type, ToastType::Info);
            assert_eq!((*use_toast.counter)(), 4);
        });
    }

    #[test]
    fn test_get_toasts() {
        with_runtime(|| {
            let use_toast = UseToast {
                toasts: Signal::new(vec![
                    Toast {
                        id: 1,
                        message: "First".to_string(),
                        toast_type: ToastType::Success,
                    },
                    Toast {
                        id: 2,
                        message: "Second".to_string(),
                        toast_type: ToastType::Error,
                    },
                ]),
                counter: Signal::new(2),
            };

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 2);
            assert_eq!(toasts[0].id, 1);
            assert_eq!(toasts[1].id, 2);
        });
    }

    #[test]
    fn test_remove_toast() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(vec![
                    Toast {
                        id: 1,
                        message: "First".to_string(),
                        toast_type: ToastType::Success,
                    },
                    Toast {
                        id: 2,
                        message: "Second".to_string(),
                        toast_type: ToastType::Error,
                    },
                    Toast {
                        id: 3,
                        message: "Third".to_string(),
                        toast_type: ToastType::Warning,
                    },
                ]),
                counter: Signal::new(3),
            };

            use_toast.remove(2);

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 2);
            assert_eq!(toasts[0].id, 1);
            assert_eq!(toasts[1].id, 3);
        });
    }

    #[test]
    fn test_remove_nonexistent_toast() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(vec![Toast {
                    id: 1,
                    message: "First".to_string(),
                    toast_type: ToastType::Success,
                }]),
                counter: Signal::new(1),
            };

            use_toast.remove(99);

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 1);
            assert_eq!(toasts[0].id, 1);
        });
    }

    #[test]
    fn test_remove_all_toasts() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(vec![
                    Toast {
                        id: 1,
                        message: "First".to_string(),
                        toast_type: ToastType::Success,
                    },
                    Toast {
                        id: 2,
                        message: "Second".to_string(),
                        toast_type: ToastType::Error,
                    },
                ]),
                counter: Signal::new(2),
            };

            use_toast.remove(1);
            use_toast.remove(2);

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 0);
        });
    }

    #[test]
    fn test_toast_workflow() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            assert_eq!(use_toast.get_toasts().len(), 0);

            use_toast.success("Operation completed");
            assert_eq!(use_toast.get_toasts().len(), 1);

            use_toast.error("Something went wrong");
            assert_eq!(use_toast.get_toasts().len(), 2);

            assert_eq!((*use_toast.counter)(), 2);

            use_toast.remove(1);
            assert_eq!(use_toast.get_toasts().len(), 1);
            assert_eq!(use_toast.get_toasts()[0].toast_type, ToastType::Error);

            use_toast.info("New info");
            use_toast.warning("Be careful");
            assert_eq!(use_toast.get_toasts().len(), 3);
            assert_eq!((*use_toast.counter)(), 4);

            use_toast.remove(2);
            use_toast.remove(3);
            use_toast.remove(4);
            assert_eq!(use_toast.get_toasts().len(), 0);
        });
    }

    #[test]
    fn test_multiple_success_toasts() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            for i in 0..5 {
                use_toast.success(&format!("Success {}", i));
            }

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 5);
            for (i, toast) in toasts.iter().enumerate() {
                assert_eq!(toast.id as usize, i + 1);
                assert_eq!(toast.message, format!("Success {}", i));
                assert_eq!(toast.toast_type, ToastType::Success);
            }
        });
    }

    #[test]
    fn test_mixed_toast_types() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            use_toast.success("Success");
            use_toast.error("Error");
            use_toast.success("Another success");
            use_toast.warning("Warning");
            use_toast.info("Info");

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 5);
            assert_eq!(toasts[0].toast_type, ToastType::Success);
            assert_eq!(toasts[1].toast_type, ToastType::Error);
            assert_eq!(toasts[2].toast_type, ToastType::Success);
            assert_eq!(toasts[3].toast_type, ToastType::Warning);
            assert_eq!(toasts[4].toast_type, ToastType::Info);
        });
    }

    #[test]
    fn test_remove_middle_toast() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(vec![
                    Toast {
                        id: 1,
                        message: "First".to_string(),
                        toast_type: ToastType::Success,
                    },
                    Toast {
                        id: 2,
                        message: "Second".to_string(),
                        toast_type: ToastType::Error,
                    },
                    Toast {
                        id: 3,
                        message: "Third".to_string(),
                        toast_type: ToastType::Warning,
                    },
                    Toast {
                        id: 4,
                        message: "Fourth".to_string(),
                        toast_type: ToastType::Info,
                    },
                ]),
                counter: Signal::new(4),
            };

            use_toast.remove(2);
            use_toast.remove(3);

            let toasts = use_toast.get_toasts();
            assert_eq!(toasts.len(), 2);
            assert_eq!(toasts[0].id, 1);
            assert_eq!(toasts[1].id, 4);
        });
    }

    #[test]
    fn test_counter_increment() {
        with_runtime(|| {
            let mut use_toast = UseToast {
                toasts: Signal::new(Vec::<Toast>::new()),
                counter: Signal::new(0),
            };

            assert_eq!((*use_toast.counter)(), 0);

            use_toast.success("Test 1");
            assert_eq!((*use_toast.counter)(), 1);

            use_toast.error("Test 2");
            assert_eq!((*use_toast.counter)(), 2);

            use_toast.warning("Test 3");
            assert_eq!((*use_toast.counter)(), 3);

            use_toast.info("Test 4");
            assert_eq!((*use_toast.counter)(), 4);

            use_toast.remove(1);
            assert_eq!((*use_toast.counter)(), 4);
        });
    }
}
