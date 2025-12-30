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
        div { style: r#"position: sticky; bottom: 0; width: 100%; border-top: 1px solid #e5e7eb; background-color: #202838;"#,
            nav { style: format!(r#"position: relative; padding-left: 1rem; padding-right: 1rem; padding-top: 0.75rem; padding-bottom: 0.75rem; {}"#, class.unwrap_or_default()),
                div { style: r#"display: flex; align-items: center; justify-content: space-between; column-gap: 1rem;"#,
                    // Items per page dropdown
                    div { style: r#"display: flex; align-items: center; column-gap: 0.5rem;"#,
                        select {
                            style: r#"display: block; width: 5rem; border-radius: 0.375rem; border: 1px solid #d1d5db; background-color: white; font-size: 0.875rem; font-weight: 500; color: #374151; box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);"#,
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
                    div { style: r#"font-size: 0.875rem; color: white;"#,
                        span { style: r#"font-weight: 500;"#, "{current_page * page_state.items_per_page + 1}" }
                        span { " - " }
                        span { style: r#"font-weight: 500;"#,
                            "{((current_page + 1) * page_state.items_per_page).min(page_state.total_items)}"
                        }
                        span { " of " }
                        span { style: r#"font-weight: 500;"#, "{page_state.total_items}" }
                    }
                    // Pagination buttons
                    div { style: r#"display: flex; align-items: center; column-gap: 0.5rem;"#,
                        // Previous button
                        button {
                            style: r#"display: flex; align-items: center; justify-content: center; border-radius: 0.375rem; padding-left: 0.75rem; padding-right: 0.75rem; padding-top: 0.5rem; padding-bottom: 0.5rem; font-size: 0.875rem; font-weight: 500; color: #6b7280; background-color: #f3f4f6;"#,
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
                                            style: format!(
                                                r#"display: flex; align-items: center; justify-content: center; border-radius: 0.375rem; padding-left: 0.75rem; padding-right: 0.75rem; padding-top: 0.5rem; padding-bottom: 0.5rem; font-size: 0.875rem; font-weight: 500; {}"#,
                                                if is_current {
                                                    r#"background-color: #4f46e5; color: white;"#
                                                } else {
                                                    r#"color: #6b7280; background-color: #f3f4f6;"#
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
                                    style: r#"display: flex; align-items: center; justify-content: center; border-radius: 0.375rem; padding-left: 0.75rem; padding-right: 0.75rem; padding-top: 0.5rem; padding-bottom: 0.5rem; font-size: 0.875rem; font-weight: 500; color: #6b7280; background-color: #f3f4f6;"#,
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
