use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Props)]
pub struct ModalConfig {
    pub modal_base: String,
    pub modal_backdrop: String,
    pub modal_container: String,
    pub modal_content: String,
    pub modal_title: String,
}

impl ModalConfig {
    pub fn default() -> Self {
        let config = Self {
            modal_base: r#"position: fixed; top: 0; right: 0; bottom: 0; left: 0; z-index: 10; overflow-y: auto;"#.to_string(),
            modal_backdrop: r#"position: fixed; top: 0; right: 0; bottom: 0; left: 0; background-color: rgba(75, 85, 99, 0.5);"#.to_string(),
            modal_container: r#"position: relative; top: 5rem; margin-left: auto; margin-right: auto; padding: 1.25rem; border: 1px solid; width: 24rem; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05); border-radius: 0.375rem; background-color: white;"#.to_string(),
            modal_content: r#"margin-top: 0.5rem; padding-left: 1.75rem; padding-right: 1.75rem; padding-top: 0.75rem; padding-bottom: 0.75rem;"#.to_string(),
            modal_title: r#"font-size: 1.125rem; line-height: 1.5rem; font-weight: 500; color: #111827;"#.to_string(),
        };
        config.to_owned()
    }

    pub fn set_modal_base(mut self, value: String) -> Self {
        self.modal_base = value;
        self
    }

    pub fn set_modal_backdrop(mut self, value: String) -> Self {
        self.modal_backdrop = value;
        self
    }

    pub fn set_modal_container(mut self, value: String) -> Self {
        self.modal_container = value;
        self
    }

    pub fn set_modal_content(mut self, value: String) -> Self {
        self.modal_content = value;
        self
    }

    pub fn set_modal_title(mut self, value: String) -> Self {
        self.modal_title = value;
        self
    }
}
