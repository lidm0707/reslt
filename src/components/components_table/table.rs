use std::fmt::Debug;

use crate::prelude::*;
use dioxus::prelude::*;
use serde::Serialize;

#[component]
pub fn DefaultTable<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    #[props(default = TableConfig::default())] 
    class: TableConfig,
    table: UseTable<T>,
    children:Element,
) -> Element {
    let class_head = class.to_owned();
    let class_main = class.to_owned();
    rsx! {
        ContainerTable {
            TableMain { class: class.table_main,
                TableHeader { class: class.table_header,
                    TableRow {
                        {
                            let class = class_head.to_owned();
                            rsx! {
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
                            for row in table.get_rows().into_iter() {
                                TableRow { class: class.to_owned().table_row,
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
            Pagination { table: table.to_owned() }
            {children}
        }
    }
}
