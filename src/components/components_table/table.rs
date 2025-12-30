use std::fmt::Debug;

use crate::prelude::*;
use dioxus::prelude::*;
use serde::Serialize;

#[component]
pub fn DefaultTable<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    #[props(default = TableConfig::default())] config: TableConfig,
    table: UseTable<T>,
    children: Element,
    #[props(default = rsx! {})] checkbox_method: Element,
    #[props(default = false)] column_check: bool,
) -> Element {
    let style_head = config.to_owned();
    let style_main = config.to_owned();
    let checkbox = use_checkbox::<T>();
    provide_context(checkbox.to_owned());
    let visible = if use_context::<UseCheckBox<T>>().get_checked_data().len() > 0 {
        true
    } else {
        false
    };
    rsx! {

        ContainerTable { style: config.table_container,
            CheckBox {
                visible,
                method: rsx! {
                    {checkbox_method}
                },
            }
            TableMain { style: config.table_main,
                TableHeader { style: config.table_header,
                    TableRow { style: config.table_row,
                        {
                            let config = style_head.to_owned();
                            rsx! {
                                if column_check {
                                    HeadCellCheckBox { style: config.to_owned(), table: table.to_owned() }
                                }
                                for col in table.get_cols().into_iter() {
                                    TableHead { style: config.to_owned().table_head,
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
                TableBody { style: config.table_body,
                    {
                        let config = style_main.to_owned();
                        let load = table.is_loading();
                        {
                            if load {
                                rsx! {
                                    for _ in 0..table.get_page_state().items_per_page.max(10) {
                                        TableRow { style: config.to_owned().table_row,
                                            if column_check {
                                                TableCell { style: config.to_owned().table_head, Skeleton {
                                                } }
                                            }
                                            for _ in table.get_cols().into_iter() {
                                                TableCell { style: config.to_owned().table_cell, Skeleton {
                                                } }
                                            }
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    for row in table.get_rows().into_iter() {
                                        TableRow { style: config.to_owned().table_row,
                                            {
                                                rsx! {
                                                    if column_check {
                                                        CellCheckBox { style: config.to_owned(), row: row.to_owned() }
                                                    }
                                                }
                                            }
                                            for col in table.get_cols().into_iter() {
                                                TableCell { style: config.to_owned().table_cell,
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
            }
            Pagination { table: table.to_owned() }
            {children}
        }
    }
}
