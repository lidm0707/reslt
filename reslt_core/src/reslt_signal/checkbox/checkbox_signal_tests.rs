#[cfg(test)]
mod tests {
    use crate::prelude::FieldAccessible;
    use crate::reslt_signal::checkbox::checkbox_signal::*;
    use dioxus::prelude::*;
    use serde::Serialize;
    use std::fmt::Debug;

    // Create a simple test type that implements all required traits
    #[derive(Serialize, Eq, Clone, PartialEq, Debug)]
    struct TestItem {
        id: u32,
        name: String,
    }

    // Implement FieldAccessible for TestItem
    impl FieldAccessible for TestItem {
        fn get_field(&self, field_name: &str) -> Option<String> {
            match field_name {
                "id" => Some(self.id.to_string()),
                "name" => Some(self.name.clone()),
                _ => None,
            }
        }
    }

    #[test]
    fn test_is_all_checked_initially_false() {
        let checkbox: UseCheckBox<TestItem> = UseCheckBox {
            data_checked: Signal::new(Vec::new()),
            is_all_checked: Signal::new(false),
        };
        assert!(!checkbox.is_all_checked());
    }

    #[test]
    fn test_set_all_checked_adds_data() {
        let mut checkbox: UseCheckBox<TestItem> = UseCheckBox {
            data_checked: Signal::new(Vec::new()),
            is_all_checked: Signal::new(false),
        };

        let data = vec![
            TestItem {
                id: 1,
                name: "Item 1".to_string(),
            },
            TestItem {
                id: 2,
                name: "Item 2".to_string(),
            },
        ];

        checkbox.set_all_checked(data);

        assert!(checkbox.is_all_checked());
        assert_eq!(checkbox.get_checked_data().len(), 2);
    }

    #[test]
    fn test_set_all_checked_removes_data() {
        let data = vec![TestItem {
            id: 1,
            name: "Item 1".to_string(),
        }];

        let mut checkbox: UseCheckBox<TestItem> = UseCheckBox {
            data_checked: Signal::new(data),
            is_all_checked: Signal::new(true),
        };

        checkbox.set_all_checked(vec![TestItem {
            id: 2,
            name: "Item 2".to_string(),
        }]);

        assert!(!checkbox.is_all_checked());
        assert_eq!(checkbox.get_checked_data().len(), 0);
    }

    #[test]
    fn test_get_checked_data() {
        let data = vec![
            TestItem {
                id: 1,
                name: "Item 1".to_string(),
            },
            TestItem {
                id: 2,
                name: "Item 2".to_string(),
            },
        ];

        let checkbox: UseCheckBox<TestItem> = UseCheckBox {
            data_checked: Signal::new(data),
            is_all_checked: Signal::new(false),
        };

        let checked = checkbox.get_checked_data();
        assert_eq!(checked.len(), 2);
        assert_eq!(checked[0].id, 1);
        assert_eq!(checked[1].id, 2);
    }

    #[test]
    fn test_set_checked_data_with_non_empty() {
        let mut checkbox: UseCheckBox<TestItem> = UseCheckBox {
            data_checked: Signal::new(vec![TestItem {
                id: 1,
                name: "Item 1".to_string(),
            }]),
            is_all_checked: Signal::new(false),
        };

        checkbox.set_checked_data(vec![TestItem {
            id: 2,
            name: "Item 2".to_string(),
        }]);

        // This behavior seems odd - if data.len() > 0, it clears the data
        assert_eq!(checkbox.get_checked_data().len(), 0);
    }

    #[test]
    fn test_set_checked_data_with_empty() {
        let mut checkbox: UseCheckBox<TestItem> = UseCheckBox {
            data_checked: Signal::new(vec![TestItem {
                id: 1,
                name: "Item 1".to_string(),
            }]),
            is_all_checked: Signal::new(false),
        };

        checkbox.set_checked_data(vec![]);

        assert_eq!(checkbox.get_checked_data().len(), 0);
    }

    #[test]
    fn test_push_checked_data() {
        let mut checkbox: UseCheckBox<TestItem> = UseCheckBox {
            data_checked: Signal::new(vec![TestItem {
                id: 1,
                name: "Item 1".to_string(),
            }]),
            is_all_checked: Signal::new(false),
        };

        checkbox.push_checked_data(TestItem {
            id: 2,
            name: "Item 2".to_string(),
        });

        let checked = checkbox.get_checked_data();
        assert_eq!(checked.len(), 2);
        assert_eq!(checked[1].id, 2);
    }

    #[test]
    fn test_remove() {
        let data = vec![
            TestItem {
                id: 1,
                name: "Item 1".to_string(),
            },
            TestItem {
                id: 2,
                name: "Item 2".to_string(),
            },
            TestItem {
                id: 3,
                name: "Item 3".to_string(),
            },
        ];

        let mut checkbox: UseCheckBox<TestItem> = UseCheckBox {
            data_checked: Signal::new(data),
            is_all_checked: Signal::new(false),
        };

        checkbox.remove(TestItem {
            id: 2,
            name: "Item 2".to_string(),
        });

        let checked = checkbox.get_checked_data();
        assert_eq!(checked.len(), 2);
        assert_eq!(checked[0].id, 1);
        assert_eq!(checked[1].id, 3);
    }

    #[test]
    fn test_remove_nonexistent_item() {
        let data = vec![
            TestItem {
                id: 1,
                name: "Item 1".to_string(),
            },
            TestItem {
                id: 2,
                name: "Item 2".to_string(),
            },
        ];

        let mut checkbox = UseCheckBox {
            data_checked: Signal::new(data),
            is_all_checked: Signal::new(false),
        };

        checkbox.remove(TestItem {
            id: 99,
            name: "Non-existent".to_string(),
        });

        assert_eq!(checkbox.get_checked_data().len(), 2);
    }

    #[test]
    fn test_checkbox_workflow() {
        let mut checkbox: UseCheckBox<TestItem> = UseCheckBox {
            data_checked: Signal::new(Vec::new()),
            is_all_checked: Signal::new(false),
        };

        // Push some items
        checkbox.push_checked_data(TestItem {
            id: 1,
            name: "Item 1".to_string(),
        });
        checkbox.push_checked_data(TestItem {
            id: 2,
            name: "Item 2".to_string(),
        });

        assert_eq!(checkbox.get_checked_data().len(), 2);
        assert!(!checkbox.is_all_checked());

        // Remove one item
        checkbox.remove(TestItem {
            id: 1,
            name: "Item 1".to_string(),
        });

        assert_eq!(checkbox.get_checked_data().len(), 1);

        // Set all checked
        checkbox.set_all_checked(vec![
            TestItem {
                id: 3,
                name: "Item 3".to_string(),
            },
            TestItem {
                id: 4,
                name: "Item 4".to_string(),
            },
        ]);

        assert!(checkbox.is_all_checked());
        assert_eq!(checkbox.get_checked_data().len(), 2);
    }
}
