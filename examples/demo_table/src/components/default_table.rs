use crate::pages::home::table::Person;
use dioxus::prelude::*;
use reslt_core::prelude::*;

#[allow(non_snake_case)]
pub fn create_person_columns() -> PropCol<Person> {
    PropCol {
        cols: vec![
            Col {
                head: "ID".to_string(),
                index: "id".to_string(),
                class: Some(
                    "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                        .to_string(),
                ),
                action: None,
            },
            Col {
                head: "Name".to_string(),
                index: "name".to_string(),
                class: Some(
                    "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                        .to_string(),
                ),
                action: None,
            },
            Col {
                head: "Age".to_string(),
                index: "age".to_string(),
                class: Some(
                    "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                        .to_string(),
                ),
                action: None,
            },
            Col {
                head: "City".to_string(),
                index: "city".to_string(),
                class: Some(
                    "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                        .to_string(),
                ),
                action: None,
            },
            Col {
                head: "Created At".to_string(),
                index: "created_at".to_string(),
                class: Some(
                    "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                        .to_string(),
                ),
                action: None,
            },
            Col {
                head: "Updated At".to_string(),
                index: "updated_at".to_string(),
                class: Some(
                    "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                        .to_string(),
                ),
                action: None,
            },
        ],
    }
}

#[component]
pub fn DefaultTable(
    table: Signal<UseTable<Person>>,
    checkbox_method: Element,
    column_check: bool,
) -> Element {
    let table_data = table.read();
    let prop_data_signal = table_data.prop_data.clone();
    let prop_col_signal = table_data.prop_col.clone();
    let mut page_state = table_data.page_state.clone();
    let is_loading_signal = table_data.is_loading;
    let sort_state = table_data.sort_state.read().clone();

    let checkbox_context = use_context::<UseCheckBox<Person>>().to_owned();

    // Pre-calculate colspan value
    let prop_col = prop_col_signal.read();
    let colspan = prop_col.cols.len() + if column_check { 1 } else { 0 };
    let colspan_str = colspan.to_string();

    rsx! {
        div {
            class: "flex flex-col space-y-4",

            // Checkbox method
            if !column_check {
                div { {checkbox_method} }
            }

            // Table container
            div {
                class: "overflow-x-auto",
                table {
                    class: "min-w-full divide-y divide-gray-200",
                    thead {
                        class: "bg-gray-50",
                        tr {
                            if column_check {
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    input {
                                        r#type: "checkbox",
                                        class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                                        checked: checkbox_context.is_all_checked(),
                                        onchange: move |e| {
                                            if e.value() == "true" {
                                                checkbox_context.set_all_checked(prop_data_signal.read().data_vec.clone());
                                            } else {
                                                checkbox_context.set_all_checked(Vec::new());
                                            }
                                        }
                                    }
                                }
                            }
                            // Clone cols to avoid lifetime issues in the loop
                            {
                                let cols_clone = prop_col.cols.clone();
                                cols_clone.iter().map(|col| {
                                    let col_class = col.class.clone().unwrap_or_default();
                                    let col_index = col.index.clone();
                                    let col_head = col.head.clone();
                                    let table_clone = table.clone();

                                    rsx! {
                                        th {
                                            class: "{col_class} cursor-pointer hover:bg-gray-100",
                                            onclick: move |_| {
                                                table_clone.with_mut(|t| {
                                                    let current_sort = t.sort_state.read().clone();
                                                    let new_descending = if let Some(current_col) = &current_sort.column {
                                                        current_col == &col_index && !current_sort.descending
                                                    } else {
                                                        true
                                                    };
                                                    *t.sort_state.write() = SortState {
                                                        column: Some(col_index.clone()),
                                                        descending: new_descending,
                                                    };
                                                });
                                            },
                                            "{col_head}"
                                            if sort_state.column.as_deref() == Some(&col_index) {
                                                span {
                                                    class: "ml-1",
                                                    if sort_state.descending {
                                                        "↓"
                                                    } else {
                                                        "↑"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }).collect::<Vec<_>>()
                            }
                        }
                    }
                    tbody {
                        class: "bg-white divide-y divide-gray-200",
                        if is_loading_signal() {
                            tr {
                                td {
                                    colspan: "{colspan_str}",
                                    class: "px-6 py-4 text-center text-gray-500",
                                    "Loading..."
                                }
                            }
                        } else if prop_data_signal.read().data_vec.is_empty() {
                            tr {
                                td {
                                    colspan: "{colspan_str}",
                                    class: "px-6 py-4 text-center text-gray-500",
                                    "No data available"
                                }
                            }
                        } else {
                            // Clone data to avoid lifetime issues in the loop
                            {
                                let rows_clone = prop_data_signal.read().data_vec.clone();
                                let cols_clone = prop_col.cols.clone();
                                rows_clone.iter().map(|row| {
                                    let row_clone = row.clone();
                                    let checkbox_context_clone = checkbox_context.clone();

                                    rsx! {
                                        tr {
                                            class: "hover:bg-gray-50",
                                            if column_check {
                                                td {
                                                    class: "px-6 py-4 whitespace-nowrap",
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                                                        checked: checkbox_context_clone.get_checked_data().contains(&row_clone),
                                                        onchange: move |_| {
                                                            let checkbox_context_inner = checkbox_context_clone.clone();
                                                            if checkbox_context_inner.get_checked_data().contains(&row_clone) {
                                                                checkbox_context_inner.remove(row_clone.clone());
                                                            } else {
                                                                checkbox_context_inner.push_checked_data(row_clone.clone());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            cols_clone.iter().map(|col| {
                                                let col_index = col.index.clone();
                                                rsx! {
                                                    td {
                                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                        {get_field_value(&row_clone, &col_index)}
                                                    }
                                                }
                                            }).collect::<Vec<_>>()
                                        }
                                    }
                                }).collect::<Vec<_>>()
                            }
                        }
                    }
                }
            }

            // Pagination
            if !is_loading_signal() && !prop_data_signal.read().data_vec.is_empty() {
                div {
                    class: "flex items-center justify-between",
                    div {
                        class: "text-sm text-gray-700",
                        "Showing {prop_data_signal.read().data_vec.len()} rows"
                    }
                    div {
                        class: "flex space-x-2",
                        button {
                            class: "px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed",
                            disabled: page_state().current_page <= 1,
                            onclick: move |_| {
                                page_state.with_mut(|p| {
                                    if p.current_page > 1 {
                                        p.current_page -= 1;
                                    }
                                });
                            },
                            "Previous"
                        }
                        span {
                            class: "px-4 py-2 text-sm text-gray-700",
                            "Page {page_state().current_page}"
                        }
                        button {
                            class: "px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed",
                            disabled: prop_data_signal.read().data_vec.len() < page_state().items_per_page,
                            onclick: move |_| {
                                page_state.with_mut(|p| {
                                    p.current_page += 1;
                                });
                            },
                            "Next"
                        }
                    }
                }
            }

            // Checkbox method (shown at bottom if column_check is enabled)
            if column_check {
                div { {checkbox_method} }
            }
        }
    }
}

fn get_field_value(person: &Person, field: &str) -> String {
    match field {
        "id" => person.id.to_string(),
        "name" => person.name.clone(),
        "age" => person.age.to_string(),
        "city" => person.city.clone(),
        "created_at" => person.created_at.clone(),
        "updated_at" => person.updated_at.clone(),
        _ => String::new(),
    }
}
