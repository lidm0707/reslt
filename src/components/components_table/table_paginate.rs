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
    let total_pages =
        (page_state.total_items + page_state.items_per_page.saturating_sub(1)) / page_state.items_per_page;
    let current_page = page_state.current_page;

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
        div { class: "sticky z-0 bottom-0 w-full  border-t border-gray-200 h-30 ",
            nav {
                class: format!(
                    "relative  border-t border-gray-200 bg-white px-4 py-3 sm:px-6 dark:bg-gray-800 dark:border-white dark:text-white {}",
                    class.unwrap_or_default(),
                ),

                div { class: "absolute flex inset-0  items-center justify-center",
                    div { class: "text-sm text-gray-500 dark:text-white",
                        span { class: "font-medium", "{current_page * page_state.items_per_page + 1}" }
                        span { " - " }
                        span { class: "font-medium",
                            "{((current_page + 1) * page_state.items_per_page).min(page_state.total_items)}"
                        }
                        span { " of " }
                        span { class: "font-medium", "{page_state.total_items}" }
                    }
                }

                div { class: "flex justify-end space-x-2",
                    // Previous page button
                    {
                        let mut table = table.to_owned();
                        rsx! {
                            button {
                                class: "relative inline-flex items-center rounded-l-md px-2 py-2 text-gray-400 dark:text-white ring-1 ring-inset ring-gray-300 hover:bg-gray-50  focus:outline-offset-0 disabled:opacity-50",
                                disabled: current_page == 0,
                                onclick: move |_| {
                                    table.set_page(current_page.saturating_sub(1));
                                },
                                span { class: "sr-only", "Previous" }
                                span { "←" }
                            }
                        }
                    }
                    {
                        page_numbers
                            .to_owned()
                            .into_iter()
                            .map(|page| {
                                let is_current = page == current_page;
                                let page = page.to_owned();
                                let table = table.to_owned();
                                rsx! {
                                    button {
                                        key: "{page}",
                                        class: if is_current { "relative  inline-flex items-center bg-indigo-600 px-4 py-2 text-sm font-semibold text-white  focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600" } else { "relative inline-flex items-center px-4 py-2 text-sm font-semibold text-gray-900 dark:text-white ring-1 ring-inset ring-gray-300 hover:bg-gray-50  focus:outline-offset-0" },
                                        onclick: move |_| {
                                            let mut table = table.to_owned();
                                            table.set_page(page);
                                        },
                                        "{page + 1}"
                                    }
                                }
                            })
                    }
                    {
                        let mut table = table.to_owned();
                        rsx! {
                            button {
                                class: "relative inline-flex items-center rounded-r-md px-2 py-2 text-gray-400 dark:text-white ring-1 ring-inset ring-gray-300 hover:bg-gray-50  focus:outline-offset-0 disabled:opacity-50",
                                disabled: current_page == total_pages.saturating_sub(1) || total_pages == 0,
                                onclick: move |_| {
                                    table.set_page((current_page + 1).min(total_pages.saturating_sub(1)));
                                },
                                span { class: "sr-only", "Next" }
                                span { "→" }
                            }
                        }
                    }
                }
            }
        }
    }
}