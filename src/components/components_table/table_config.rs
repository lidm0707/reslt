use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Props)]
pub struct TableConfig {
    pub table_container: String,
    pub table_main: String,
    pub table_header: String,
    pub table_head: String,
    pub table_body: String,
    pub table_row: String,
    pub table_cell: String,
}

impl TableConfig {
    pub fn default() -> Self {
        let config = Self {
            table_container: r#"position: absolute; overflow-y: scroll; height: 100%; width: 100vw;"#.to_string(),
            table_main: r#"width: 100%; height: 100%; font-size: 0.875rem; text-align: left; color: white;"#.to_string(),
            table_header: r#"position: sticky; z-index: 10; top: 0; font-size: 0.75rem; text-transform: uppercase; color: white; background-color: #f9fafb;"#.to_string(),
            table_head: r#"padding: 0.75rem 1.5rem 0.75rem 1.5rem;"#.to_string(),
            table_body: "".to_string(),
            table_row: r#"background-color: #202838; border-bottom: 1px solid #e5e7eb;"#.to_string(),
            table_cell: r#"padding: 0.75rem 1.5rem 0.75rem 1.5rem;"#.to_string(),
        };
        config.to_owned()
    }

    pub fn set_table_container(mut self, value: String) -> Self {
        self.table_container = value;
        self
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
