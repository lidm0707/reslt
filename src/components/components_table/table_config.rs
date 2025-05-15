use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Props)]
pub struct TableConfig {
    pub table_main: String,
    pub table_header: String,
    pub table_head: String,
    pub table_body: String,
    pub table_row: String,
    pub table_cell: String,
}

impl TableConfig {
    pub fn default() -> Self {      
        let config =  Self {
            table_main: "w-full h-100 text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400".to_string(),
            table_header: "sticky z-3 top-0 text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400 ".to_string(),
            table_head: "px-6 py-3".to_string(),
            table_body: "".to_string(),
            table_row: "bg-white border-b border-gray-200 hover:bg-gray-50 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-600".to_string(),
            table_cell: "px-6 py-3".to_string(),
        };
        config.to_owned()
    }

    pub fn set_table_main(mut self, value: String) -> Self {
        self.table_main = value;
        self
    }

    pub fn set_table_header(mut self, value: String) -> Self {
        self.table_header = value;
        self
    }

    pub fn set_table_head(mut self, value: String) -> Self {
        self.table_head = value;
        self
    }

    pub fn set_table_body(mut self, value: String) -> Self {
        self.table_body = value;
        self
    }

    pub fn set_table_row(mut self, value: String) -> Self {
        self.table_row = value;
        self
    }

    pub fn set_table_cell(mut self, value: String) -> Self {
        self.table_cell = value;
        self
    }
}

