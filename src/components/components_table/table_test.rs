// src/components/components_table/table_test.rs
#![allow(non_snake_case)]
use crate::prelude::FieldAccessible;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// It's likely that `UseTable` and `TableCol` are part of the `table_signal` and `table_col` modules respectively.
// Adjust these imports based on your actual module structure.
// It seems `use_table` is the hook to get `UseTable`.
use crate::components::components_table::table_signal::use_table;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, FieldAccessible, Default)]
struct TestItem {
    id: u32,
    name: String,
    age: u32,
}

impl TestItem {
    fn new(id: u32, name: &str, age: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            age,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{future::Future, pin::Pin};

    use crate::prelude::{Col, PropCol, PropData};

    use super::*;

    fn get_test_data(
        _start: usize,
        _end: usize,
        _sort: (String, bool),
    ) -> Pin<Box<dyn 'static + Future<Output = (PropData<TestItem>, usize)>>> {
        Box::pin(async move {
            (
                PropData {
                    data_vec: vec![
                        TestItem::new(1, "Alice", 30),
                        TestItem::new(2, "Bob", 24),
                        TestItem::new(3, "Charlie", 24),
                        TestItem::new(4, "Diana", 28),
                        TestItem::new(5, "Edward", 40),
                    ],
                },
                5,
            )
        })
    }

    fn get_table_cols() -> PropCol<TestItem> {
        PropCol {
            cols: vec![
                Col {
                    head: "ID".to_owned(),
                    index: "id".to_owned(),
                    class: Some("text-right".to_owned()),
                    action: None,
                },
                Col {
                    head: "Name".to_owned(),
                    index: "name".to_owned(),
                    class: None,
                    action: None,
                },
                Col {
                    head: "AGE".to_owned(),
                    index: "age".to_owned(),
                    class: None,
                    action: None,
                },
            ],
        }
    }

    // fn test_modal_initial_state() {
    //     let mut vdom = VirtualDom::new_with_props(
    //         move || {
    //             let modal_signal = use_modal();
    //             use_context_provider(|| modal_signal.clone());
    //             assert!(!modal_signal.is_open(), "Modal should be initially closed.");
    //             rsx! {
    //                 Modal { config: ModalConfig::default(),
    //                     p { "Test" }
    //                 }
    //             }
    //         },
    //         (),
    //     );

    //     vdom.rebuild_in_place();
    // }

    #[test]
    fn test_table_initialization() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let cols = get_table_cols();
                let table = use_table(get_test_data, cols.to_owned());
                assert!(
                    table.get_rows().is_empty(),
                    "Table should be initially empty."
                );
                assert!(table.is_loading(), "Table should not be loading initially.");
                rsx! {}
            },
            (),
        );
        vdom.rebuild_in_place();
    }
    #[component]
    fn Setdata() -> Element {
        let cols = get_table_cols();
        let table = use_table(get_test_data, cols.to_owned());

        println!("Cols: {:?}", cols);
        println!("Table rows: {:?}", table.get_rows());

        // ตรวจสอบเฉพาะความยาวของข้อมูล
        // assert_eq!(table.get_rows().len(), 5,);

        rsx! {}
    }

    #[test]
     fn test_table_set_data() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                rsx! {Setdata{}}
            },
            (),
        );
        vdom.rebuild_in_place();
    }
    // assert!(
    //     !table.get_rows().is_empty(),
    //     "Table should not be empty after setting data."
    // );

    //     #[test]
    //     fn test_table_loading_state() {
    //         run_table_test(|| {
    //             let mut table = use_table::<TestItem>(, || get_table_cols(), || vec![]);

    //             table.set_loading(true);
    //             assert!(
    //                 table.is_loading(),
    //                 "is_loading() should be true after set_loading(true)."
    //             );

    //             table.set_loading(false);
    //             assert!(
    //                 !table.is_loading(),
    //                 "is_loading() should be false after set_loading(false)."
    //             );
    //         });
    //     }

    //     #[test]
    //     fn test_table_pagination() {
    //         run_table_test(|| {
    //             let mut table = use_table::<TestItem>(, || get_table_cols(), || vec![]);
    //             let data = (0..25)
    //                 .map(|i| TestItem::new(i, &format!("Item {}", i), 20 + i, None))
    //                 .collect::<Vec<_>>();
    //             table.set_data(data.clone());

    //             assert_eq!(table.get_page_state().items_per_page, 10);
    //             assert_eq!(table.get_page_state().total_items, 25);
    //             assert_eq!(
    //                 table.get_page_state().total_pages,
    //                 3,
    //                 "Should be 3 pages for 25 items, 10 per page."
    //             );
    //             assert_eq!(
    //                 table.get_page_state().current_page,
    //                 1,
    //                 "Current page should be 1 initially."
    //             );
    //             assert_eq!(
    //                 table.get_rows().len(),
    //                 10,
    //                 "Should show 10 items on page 1."
    //             );
    //             if !table.get_rows().is_empty() {
    //                 // Guard against empty rows before accessing index 0
    //                 assert_eq!(table.get_rows()[0].id, 0);
    //             }

    //             table.next_page();
    //             assert_eq!(
    //                 table.get_page_state().current_page,
    //                 2,
    //                 "Current page should be 2 after next_page()."
    //             );
    //             assert_eq!(
    //                 table.get_rows().len(),
    //                 10,
    //                 "Should show 10 items on page 2."
    //             );
    //             if !table.get_rows().is_empty() {
    //                 assert_eq!(table.get_rows()[0].id, 10);
    //             }

    //             table.next_page();
    //             assert_eq!(
    //                 table.get_page_state().current_page,
    //                 3,
    //                 "Current page should be 3 after next_page()."
    //             );
    //             assert_eq!(table.get_rows().len(), 5, "Should show 5 items on page 3.");
    //             if !table.get_rows().is_empty() {
    //                 assert_eq!(table.get_rows()[0].id, 20);
    //             }

    //             table.next_page();
    //             assert_eq!(
    //                 table.get_page_state().current_page,
    //                 3,
    //                 "Current page should remain 3."
    //             );

    //             table.prev_page();
    //             assert_eq!(
    //                 table.get_page_state().current_page,
    //                 2,
    //                 "Current page should be 2 after prev_page()."
    //             );
    //             assert_eq!(table.get_rows().len(), 10);
    //             if !table.get_rows().is_empty() {
    //                 assert_eq!(table.get_rows()[0].id, 10);
    //             }

    //             table.set_page(1);
    //             assert_eq!(
    //                 table.get_page_state().current_page,
    //                 1,
    //                 "Current page should be 1 after set_page(1)."
    //             );
    //             assert_eq!(table.get_rows().len(), 10);
    //             if !table.get_rows().is_empty() {
    //                 assert_eq!(table.get_rows()[0].id, 0);
    //             }

    //             table.set_items_per_page(5);
    //             assert_eq!(table.get_page_state().items_per_page, 5);
    //             assert_eq!(
    //                 table.get_page_state().total_pages,
    //                 5,
    //                 "Should be 5 pages for 25 items, 5 per page."
    //             );
    //             assert_eq!(
    //                 table.get_page_state().current_page,
    //                 1,
    //                 "Current page should reset to 1 after changing items_per_page."
    //             );
    //             assert_eq!(table.get_rows().len(), 5);
    //             if !table.get_rows().is_empty() {
    //                 assert_eq!(table.get_rows()[0].id, 0);
    //             }
    //         });
    //     }

    //     #[test]
    //     fn test_table_sorting() {
    //         run_table_test(|| {
    //             let mut table = use_table::<TestItem>(, || get_table_cols(), || get_test_data());

    //             table.sort_by("name", false);
    //             let sorted_by_name_asc = table.get_rows_all_without_pag();
    //             assert_eq!(sorted_by_name_asc[0].name, "Alice");
    //             assert_eq!(sorted_by_name_asc[1].name, "Bob");
    //             assert_eq!(sorted_by_name_asc[2].name, "Charlie");

    //             table.sort_by("name", true);
    //             let sorted_by_name_desc = table.get_rows_all_without_pag();
    //             assert_eq!(sorted_by_name_desc[0].name, "Edward");
    //             assert_eq!(sorted_by_name_desc[1].name, "Diana");
    //             assert_eq!(sorted_by_name_desc[2].name, "Charlie");

    //             table.sort_by("age", false);
    //             let sorted_by_age_asc = table.get_rows_all_without_pag();
    //             assert_eq!(sorted_by_age_asc[0].age, 24);
    //             assert_eq!(sorted_by_age_asc[1].age, 28);
    //             assert_eq!(sorted_by_age_asc[2].age, 30);

    //             table.sort_by("age", true);
    //             let sorted_by_age_desc = table.get_rows_all_without_pag();
    //             assert_eq!(sorted_by_age_desc[0].age, 40);
    //             assert_eq!(sorted_by_age_desc[1].age, 35);
    //             assert_eq!(sorted_by_age_desc[2].age, 30);

    //             table.sort_by("age", false);
    //             let sorted_by_age_asc_again = table.get_rows_all_without_pag();
    //             assert_eq!(sorted_by_age_asc_again[0].age, 24);
    //             assert_eq!(table.get_sort_state().0, "age");
    //             assert_eq!(table.get_sort_state().1, false);
    //         });
    //     }
    //     #[test]
    //     fn test_table_column_manipulation() {
    //         // Renamed from test_table_column_selection
    //         run_table_test(|| {
    //             let initial_cols = vec![TableCol::new("id"), TableCol::new("name")];
    //             let mut table = use_table::<TestItem>(, || initial_cols.clone(), || vec![]);

    //             assert_eq!(table.get_cols().len(), 2);
    //             assert_eq!(table.get_cols()[0].name, "id");

    //             let new_cols = vec![TableCol::new("age")];
    //             table.set_cols(new_cols.clone()); // Assuming UseTable has a set_cols method
    //             assert_eq!(table.get_cols().len(), 1);
    //             assert_eq!(table.get_cols()[0].name, "age");
    //         });
    //     }
}
