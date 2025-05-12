use std::fmt::Debug;

use crate::prelude::*;
use dioxus::prelude::*;
use serde::Serialize;

#[component]
pub fn DefaultTable<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    #[props(default = TableConfig::default())] class: TableConfig,
    table: UseTable<T>,
    children: Element,
    #[props(default = rsx! {})] checkbox_method: Element,
    #[props(default = false)] column_check:bool,
) -> Element {
    let class_head = class.to_owned();
    let class_main = class.to_owned();
    let checkbox = use_checkbox::<T>();
    provide_context(checkbox.to_owned());
    let visible = if  use_context::<UseCheckBox<T>>().get_checked_data().len() > 0 {true} else {false};
    rsx! {
        ContainerTable {
            CheckBox {
                visible,
                method: rsx! {
                    {checkbox_method}
                },
            }
            TableMain { class: class.table_main,
                TableHeader { class: class.table_header,
                    TableRow {
                        {
                            let class = class_head.to_owned();
                            rsx! {
                                if column_check {
                                    TableHead { class: class.to_owned().table_head,
                                        {
                                            let data = table.to_owned().get_rows();
                                            rsx! {
                                                input {
                                                    r#type: "checkbox",
                                                    onchange: move |_| {
                                                        use_context::<UseCheckBox<T>>().set_all_checked(data.to_owned());
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                                for col in table.get_cols().into_iter() {
                                    TableHead { class: class.to_owned().table_head,
                                        {
                                            let value = table.to_owned();
                                            rsx! {
                                                ArrowSort { table: value.to_owned(), col: col.to_owned() }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                TableBody {
                    {
                        let class = class_main.to_owned();
                        rsx! {
                            if table.is_loading() {
                                // Show skeleton rows with the same number as the current page size
                                for _ in 0..table.get_page_state().items_per_page.min(10) {
                                    TableRow { class: class.to_owned().table_row,
                                        if column_check {
                                            TableCell { class: class.to_owned().table_head, Skeleton {
                                            } }
                                        }
                                        for _ in table.get_cols().into_iter() {
                                            TableCell { class: class.to_owned().table_cell, Skeleton {
                                            } }
                                        }
                                    }
                                }
                            } else {
                                // Show actual data when loaded
                                for row in table.get_rows().into_iter() {
                                    TableRow { class: class.to_owned().table_row,
                                        {
                                            rsx! {
                                                if column_check {
                                                    {
                                                        let row = row.to_owned();
                                                        let mut checked = use_signal(|| false);
                                                        let update_check_all = use_context::<UseCheckBox<T>>().is_all_checked();
                                                        use_effect(
                                                            use_reactive!(| (update_check_all) | { checked.set(update_check_all); }),
                                                        );
                                                        rsx! {
                                                            TableCell { class: class.to_owned().table_cell,
                                                                input {
                                                                    r#type: "checkbox",
                                                                    checked: checked(),
                                                                    onchange: move |_| {
                                                                        if !checked() {
                                                                            use_context::<UseCheckBox<T>>().to_owned().push_checked_data(row.to_owned());
                                                                            checked.set(true);
                                                                        } else {
                                                                            use_context::<UseCheckBox<T>>().to_owned().remove(row.to_owned());
                                                                            checked.set(false);
                                                                        }
                                                                    },
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        for col in table.get_cols().into_iter() {
                                            TableCell { class: class.to_owned().table_cell,
                                                {
                                                    let row_copy = row.to_owned();
                                                    let col_copy = col.to_owned();
                                                    rsx! {
                                                        DefaultChildren { row: row_copy, col: col_copy }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Pagination { table: table.to_owned() }
            {children}
        }
    }
}
