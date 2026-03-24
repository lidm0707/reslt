#[cfg(test)]
mod tests {
    use crate::prelude::FieldAccessible;
    use crate::reslt_signal::table::table_col::{Col, PropCol};
    use crate::reslt_signal::table::table_data::PropData;
    use crate::reslt_signal::table::table_signal::*;
    use dioxus::prelude::*;
    use serde::Serialize;
    use std::fmt::Debug;

    // Create a test type that implements all required traits
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
    fn test_page_state_default() {
        let page_state = PageState::default();
        assert_eq!(page_state.current_page, 0);
        assert_eq!(page_state.items_per_page, 0);
        assert_eq!(page_state.total_items, 0);
    }

    #[test]
    fn test_page_state_initialization() {
        let page_state = PageState {
            current_page: 5,
            items_per_page: 20,
            total_items: 100,
        };
        assert_eq!(page_state.current_page, 5);
        assert_eq!(page_state.items_per_page, 20);
        assert_eq!(page_state.total_items, 100);
    }

    #[test]
    fn test_sort_state_default() {
        let sort_state = SortState::default();
        assert_eq!(sort_state.column, None);
        assert_eq!(sort_state.descending, true);
    }

    #[test]
    fn test_sort_state_initialization() {
        let sort_state = SortState {
            column: Some("name".to_string()),
            descending: false,
        };
        assert_eq!(sort_state.column, Some("name".to_string()));
        assert_eq!(sort_state.descending, false);
    }

    #[test]
    fn test_sort_state_partial_eq() {
        let sort_state1 = SortState {
            column: Some("name".to_string()),
            descending: false,
        };
        let sort_state2 = SortState {
            column: Some("name".to_string()),
            descending: false,
        };
        assert_eq!(sort_state1, sort_state2);
    }

    #[test]
    fn test_sort_state_partial_eq_different() {
        let sort_state1 = SortState {
            column: Some("name".to_string()),
            descending: false,
        };
        let sort_state2 = SortState {
            column: Some("id".to_string()),
            descending: false,
        };
        assert_ne!(sort_state1, sort_state2);
    }

    #[test]
    fn test_use_table_sort_by_field_new_field() {
        let mut use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol {
                cols: vec![
                    Col {
                        head: "ID".to_string(),
                        index: "id".to_string(),
                        class: None,
                        action: None,
                    },
                    Col {
                        head: "Name".to_string(),
                        index: "name".to_string(),
                        class: None,
                        action: None,
                    },
                ],
            }),
            sort_state: Signal::new(SortState {
                column: Some("id".to_string()),
                descending: true,
            }),
            page_state: Signal::new(PageState {
                current_page: 0,
                items_per_page: 10,
                total_items: 2,
            }),
            is_loading: Signal::new(false),
        };

        use_table.sort_by_field("name");
        assert_eq!(use_table.get_sort_col(), "name".to_string());
        assert_eq!(use_table.get_sort_state(), false); // New field should be ascending
    }

    #[test]
    fn test_use_table_sort_by_field_same_field() {
        let mut use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol {
                cols: vec![Col {
                    head: "ID".to_string(),
                    index: "id".to_string(),
                    class: None,
                    action: None,
                }],
            }),
            sort_state: Signal::new(SortState {
                column: Some("name".to_string()),
                descending: false,
            }),
            page_state: Signal::new(PageState {
                current_page: 0,
                items_per_page: 10,
                total_items: 1,
            }),
            is_loading: Signal::new(false),
        };

        use_table.sort_by_field("name");
        assert_eq!(use_table.get_sort_col(), "name".to_string());
        assert_eq!(use_table.get_sort_state(), true); // Should toggle to descending
    }

    #[test]
    fn test_use_table_get_rows() {
        let use_table = UseTable {
            prop_data: Signal::new(PropData {
                data_vec: vec![
                    TestItem {
                        id: 1,
                        name: "Item 1".to_string(),
                    },
                    TestItem {
                        id: 2,
                        name: "Item 2".to_string(),
                    },
                ],
            }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState::default()),
            page_state: Signal::new(PageState::default()),
            is_loading: Signal::new(false),
        };

        let rows = use_table.get_rows();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].id, 1);
        assert_eq!(rows[1].id, 2);
    }

    #[test]
    fn test_use_table_get_cols() {
        let use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol {
                cols: vec![
                    Col {
                        head: "ID".to_string(),
                        index: "id".to_string(),
                        class: None,
                        action: None,
                    },
                    Col {
                        head: "Name".to_string(),
                        index: "name".to_string(),
                        class: None,
                        action: None,
                    },
                ],
            }),
            sort_state: Signal::new(SortState::default()),
            page_state: Signal::new(PageState::default()),
            is_loading: Signal::new(false),
        };

        let cols = use_table.get_cols();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0].head, "ID");
        assert_eq!(cols[1].head, "Name");
    }

    #[test]
    fn test_use_table_get_sort_state() {
        let use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState {
                column: Some("name".to_string()),
                descending: false,
            }),
            page_state: Signal::new(PageState::default()),
            is_loading: Signal::new(false),
        };

        assert_eq!(use_table.get_sort_state(), false);
    }

    #[test]
    fn test_use_table_get_sort_col() {
        let use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState {
                column: Some("name".to_string()),
                descending: true,
            }),
            page_state: Signal::new(PageState::default()),
            is_loading: Signal::new(false),
        };

        assert_eq!(use_table.get_sort_col(), "name".to_string());
    }

    #[test]
    fn test_use_table_get_sort_col_none() {
        let use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState {
                column: None,
                descending: true,
            }),
            page_state: Signal::new(PageState::default()),
            is_loading: Signal::new(false),
        };

        assert_eq!(use_table.get_sort_col(), String::new());
    }

    #[test]
    fn test_use_table_get_page_state() {
        let use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState::default()),
            page_state: Signal::new(PageState {
                current_page: 0,
                items_per_page: 10,
                total_items: 100,
            }),
            is_loading: Signal::new(false),
        };

        let page_state = use_table.get_page_state();
        assert_eq!(page_state.current_page, 5);
        assert_eq!(page_state.items_per_page, 20);
        assert_eq!(page_state.total_items, 100);
    }

    #[test]
    fn test_use_table_set_page() {
        let mut use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState::default()),
            page_state: Signal::new(PageState {
                current_page: 0,
                items_per_page: 10,
                total_items: 100,
            }),
            is_loading: Signal::new(false),
        };

        use_table.set_page(10);
        let page_state = use_table.get_page_state();
        assert_eq!(page_state.current_page, 10);
    }

    #[test]
    fn test_use_table_set_items_per_page() {
        let mut use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState::default()),
            page_state: Signal::new(PageState {
                current_page: 0,
                items_per_page: 10,
                total_items: 100,
            }),
            is_loading: Signal::new(false),
        };

        use_table.set_items_per_page(25);
        let page_state = use_table.get_page_state();
        assert_eq!(page_state.items_per_page, 25);
    }

    #[test]
    fn test_use_table_set_loading() {
        let mut use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState::default()),
            page_state: Signal::new(PageState::default()),
            is_loading: Signal::new(false),
        };

        assert!(!use_table.is_loading());
        use_table.set_loading(true);
        assert!(use_table.is_loading());
        use_table.set_loading(false);
        assert!(!use_table.is_loading());
    }

    #[test]
    fn test_use_table_is_loading() {
        let use_table_loading: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState::default()),
            page_state: Signal::new(PageState::default()),
            is_loading: Signal::new(true),
        };

        let use_table_not_loading: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol { cols: vec![] }),
            sort_state: Signal::new(SortState::default()),
            page_state: Signal::new(PageState::default()),
            is_loading: Signal::new(false),
        };

        assert!(use_table_loading.is_loading());
        assert!(!use_table_not_loading.is_loading());
    }

    #[test]
    fn test_prop_data_display() {
        let prop_data = PropData {
            data_vec: vec![
                TestItem {
                    id: 1,
                    name: "Item 1".to_string(),
                },
                TestItem {
                    id: 2,
                    name: "Item 2".to_string(),
                },
            ],
        };

        let display_str = format!("{}", prop_data);
        assert!(display_str.contains("TestItem"));
    }

    #[test]
    fn test_table_workflow() {
        let mut use_table: UseTable<TestItem> = UseTable {
            prop_data: Signal::new(PropData { data_vec: vec![] }),
            prop_col: Signal::new(PropCol {
                cols: vec![
                    Col {
                        head: "ID".to_string(),
                        index: "id".to_string(),
                        class: None,
                        action: None,
                    },
                    Col {
                        head: "Name".to_string(),
                        index: "name".to_string(),
                        class: None,
                        action: None,
                    },
                ],
            }),
            sort_state: Signal::new(SortState::default()),
            page_state: Signal::new(PageState {
                current_page: 0,
                items_per_page: 10,
                total_items: 0,
            }),
            is_loading: Signal::new(false),
        };

        // Check initial state
        assert!(!use_table.is_loading());
        assert_eq!(use_table.get_rows().len(), 0);
        assert_eq!(use_table.get_cols().len(), 2);

        // Sort by field
        use_table.sort_by_field("name");
        assert_eq!(use_table.get_sort_col(), "name");
        assert!(!use_table.get_sort_state());

        // Set page
        use_table.set_page(2);
        let page_state = use_table.get_page_state();
        assert_eq!(page_state.current_page, 2);

        // Set items per page
        use_table.set_items_per_page(20);
        let page_state = use_table.get_page_state();
        assert_eq!(page_state.items_per_page, 20);

        // Set loading
        use_table.set_loading(true);
        assert!(use_table.is_loading());
        use_table.set_loading(false);
        assert!(!use_table.is_loading());
    }
}
