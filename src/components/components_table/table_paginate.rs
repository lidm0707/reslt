use std::fmt::Debug;

use crate::prelude::*;
use dioxus::prelude::*;
use serde::Serialize;

#[component]
pub fn Pagination<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    table: UseTable<T>,
    class: Option<String>,
) -> Element {
    let page_state = table.get_page_state();
    let total_pages = (page_state.total_items + page_state.items_per_page.saturating_sub(1))
        / page_state.items_per_page;
    let current_page = page_state.current_page;
    let mut items_per_page = table.to_owned();
    // Generate page numbers to display
    let mut page_numbers = vec![];

    page_numbers.extend([
        current_page.saturating_sub(1),
        current_page,
        (current_page + 1).min(total_pages.saturating_sub(1)),
    ]);

    if current_page == 0 && total_pages > 2 {
        page_numbers.push(2);
    }

    if current_page == (total_pages.saturating_sub(1)) && total_pages > 2 {
        page_numbers.insert(0, total_pages.saturating_sub(3));
    }
    println!("total_pages: {:?}", total_pages);
    page_numbers.sort();
    page_numbers.dedup();

    rsx! {
        div { class: "sticky bottom-0  w-full border-t border-gray-200 bg-white dark:bg-gray-800 dark:border-gray-700",
            nav { class: format!("relative px-4 py-3 sm:px-6 {}", class.unwrap_or_default()),
                div { class: "flex items-center justify-between space-x-4",
                    // Items per page dropdown
                    div { class: "flex items-center space-x-2",
                        select {
                            class: "block w-20 rounded-md border-gray-300 bg-white text-sm font-medium text-gray-700 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:bg-gray-700 dark:text-gray-300 dark:border-gray-600",
                            disabled: table.is_loading(),
                            onchange: move |evt| {
                                items_per_page.set_loading(true);
                                items_per_page.set_items_per_page(evt.data.value().parse().unwrap_or(10));
                            },
                            option { value: "10", "10" }
                            option { value: "50", "50" }
                            option { value: "100", "100" }
                        }
                    }
                    // Pagination information
                    div { class: "text-sm text-gray-500 dark:text-gray-400",
                        span { class: "font-medium", "{current_page * page_state.items_per_page + 1}" }
                        span { " - " }
                        span { class: "font-medium",
                            "{((current_page + 1) * page_state.items_per_page).min(page_state.total_items)}"
                        }
                        span { " of " }
                        span { class: "font-medium", "{page_state.total_items}" }
                    }
                    // Pagination buttons
                    div { class: "flex items-center space-x-2",
                        // Previous button
                        button {
                            class: "flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium text-gray-500 bg-gray-100 hover:bg-gray-200 dark:text-gray-400 dark:bg-gray-700 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed",
                            disabled: table.is_loading() || current_page == 0,
                            onclick: move |_| {
                                table.set_page(current_page.saturating_sub(1));
                            },
                            span { "←" }
                        }
                        // Page numbers
                        {
                            page_numbers
                                .to_owned()
                                .into_iter()
                                .map(|page| {
                                    let is_current = page == current_page;
                                    let page = page.to_owned();
                                    let mut table = table.to_owned();
                                    rsx! {
                                        button {
                                            key: "{page}",
                                            class: format!(
                                                "flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium {}",
                                                if is_current {
                                                    "bg-indigo-600 text-white focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                                                } else {
                                                    "text-gray-500 bg-gray-100 hover:bg-gray-200 dark:text-gray-400 dark:bg-gray-700 dark:hover:bg-gray-600"
                                                },
                                            ),
                                            disabled: table.is_loading() || is_current,
                                            onclick: move |_| {
                                                table.set_page(page);
                                            },
                                            "{page + 1}"
                                        }
                                    }
                                })
                        }
                        // Next button
                        {
                            let mut table = table.to_owned();
                            rsx! {
                                button {
                                    class: "flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium text-gray-500 bg-gray-100 hover:bg-gray-200 dark:text-gray-400 dark:bg-gray-700 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed",
                                    disabled: table.is_loading() || current_page == total_pages.saturating_sub(1)
                                        || total_pages == 0,
                                    onclick: move |_| {
                                        table.set_page((current_page + 1).min(total_pages.saturating_sub(1)));
                                    },
                                    span { "→" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
}
