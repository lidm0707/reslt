use std::fmt::Debug;

use crate::prelude::*;
use dioxus::prelude::*;
use serde::Serialize;

#[component]
pub fn ArrowSort<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    class: Option<String>,
    table: UseTable<T>,
    col: Col<T>,
) -> Element {
    rsx! {

        {
            let mut value = table.to_owned();
            rsx! {
                div {
                    onclick: move |_| {
                        value.sort_by_field(&col.index);
                    },
                    {col.head}
                    match (value.get_sort_col(), value.get_sort_state()) {
                        (col_index, true) if col_index == col.index => {
                            rsx! {
                                span { class: "ml-1", "▲" }
                            }
                        }
                        (col_index, false) if col_index == col.index => {
                            rsx! {
                                span { class: "ml-1", "▼" }
                            }
                        }
                        _ => rsx! {
                            span {}
                        },
                    }
                }
            }
        }
    }
}