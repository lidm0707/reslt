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
            modal_base: "fixed inset-0 z-10 overflow-y-auto".to_string(),
            modal_backdrop: "fixed inset-0 bg-gray-600/50".to_string(),
            modal_container: "relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white".to_string(),
            modal_content: "mt-2 px-7 py-3".to_string(),
            modal_title: "text-lg leading-6 font-medium text-gray-900".to_string(),
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