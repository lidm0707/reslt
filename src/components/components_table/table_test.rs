#![allow(non_snake_case)]
use crate::prelude::FieldAccessible;
use async_std::task::sleep;
use dioxus::prelude::*;
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::LazyLock};

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
static TEST_ARRAY: LazyLock<Mutex<Vec<TestItem>>> = LazyLock::new(|| {
    Mutex::new(vec![
        TestItem::new(1, "Alice", 30),
        TestItem::new(2, "Bob", 24),
        TestItem::new(3, "Charlie", 24),
        TestItem::new(4, "Diana", 28),
        TestItem::new(5, "Edward", 40),
    ])
});

#[cfg(test)]
mod tests {
    use std::{future::Future, pin::Pin, time::Duration};

    use dioxus::dioxus_core::Mutation;

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
                    data_vec: TEST_ARRAY.lock().await.clone(),
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

    async fn test_table_initialization() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let cols = get_table_cols();
                let table = use_table(get_test_data, cols.to_owned(), None);

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

    async fn test_table_set_data() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let cols = get_table_cols();
                let table = use_table(get_test_data, cols.to_owned(), None);
                rsx! {for i in table.get_rows(){
                    "{i.name}"
                "{i.age}"
                "{i.id}"

                }}
            },
            (),
        );

        vdom.rebuild_in_place();
        sleep(Duration::from_secs(1)).await;
        let mutations = vdom.render_immediate_to_vec();
        assert_eq!(
            mutations.edits.len(),
            16,
            "The number of mutations does not match the expected value."
        );
        let expected_values = [
            "Alice", "30", "1", "Bob", "24", "2", "Charlie", "24", "3", "Diana", "28", "4",
            "Edward", "40", "5",
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            match &mutations.edits[i] {
                Mutation::CreateTextNode { value, id: _ } => {
                    assert_eq!(
                        value.as_str(),
                        *expected,
                        "ค่าที่ edit ลำดับ {} ไม่ตรง (found: '{}', expected: '{}')",
                        i,
                        value,
                        expected
                    );
                }
                other => println!("edit ลำดับ {} ไม่ใช่ CreateTextNode: {:?}", i, other),
            }
        }
    }

    #[test]
    fn test_table_column_manipulation() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let cols = get_table_cols();
                let table = use_table(get_test_data, cols.to_owned(), None);
                rsx! {for i in table.get_cols(){
                    "{i.head}"
                }}
            },
            (),
        );

        let first = vdom.rebuild_to_vec();
        println!("First: {:?}", first.edits);
        assert_eq!(
            first.edits.len(),
            4,
            "The number of mutations does not match the expected value."
        );
    }

    async fn test_table_loading_state() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let cols = get_table_cols();
                let table = use_table(get_test_data, cols.to_owned(), None);
                rsx! {"{table.is_loading()}"}
            },
            (),
        );

        let first = vdom.rebuild_to_vec();
        println!("First: {:?}", first.edits);
        match &first.edits[0] {
            Mutation::CreateTextNode { value, id: _ } => {
                assert_eq!(value.as_str(), "true",);
            }
            other => println!("edit ลำดับ {} ไม่ใช่ CreateTextNode: {:?}", 0, other),
        }
        sleep(Duration::from_secs(1)).await;

        let mutations = vdom.render_immediate_to_vec();
        println!("Second: {:?}", mutations.edits);
        match &mutations.edits[0] {
            Mutation::SetText { value, id: _ } => {
                assert_eq!(value.as_str(), "false",);
            }
            other => panic!("edit ลำดับ {} ไม่ใช่ CreateTextNode: {:?}", 0, other),
        }
    }

    #[test]
    fn test_table_pagination() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let cols = get_table_cols();
                let table = use_table(get_test_data, cols.to_owned(), None);
                // assert_eq!(table.get_page_state().items_per_page, 10);

                rsx! {"{table.get_page_state().total_items}"}
            },
            (),
        );

        vdom.rebuild_to_vec();
        let second = vdom.render_immediate_to_vec();

        match &second.edits[0] {
            Mutation::SetText { value, id: _ } => {
                assert_eq!(value.as_str(), "5",);
            }
            other => panic!("edit ลำดับ {} ไม่ใช่ SetText: {:?}", 0, other),
        }
    }

    #[test]
    fn test_table_sorting() {
        let mut vdom = VirtualDom::new_with_props(
            move || {
                let cols = get_table_cols();
                let mut table = use_table(get_test_data, cols.to_owned(), None);
                rsx! {{table.sort_by_field("name");}, "{table.get_sort_state()}"}
            },
            (),
        );

        vdom.rebuild_to_vec();
        let second = vdom.render_immediate_to_vec();
        println!("start: {:?}", second);
        match &second.edits[0] {
            Mutation::SetText { value, id: _ } => {
                assert_eq!(value.as_str(), "true",);
            }
            other => panic!("edit ลำ {} ไม่ใช่ SetText: {:?}", 0, other),
        }
    }
}
