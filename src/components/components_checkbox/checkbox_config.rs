use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Props)]
pub struct CheckboxConfig {
    pub checkbox_base: String,
    pub checkbox_container: String,
    pub checkbox_input: String,
    pub checkbox_label: String,
    pub checkbox_checked: String,
    pub checkbox_unchecked: String,
    pub checkbox_disabled: String,
}

impl CheckboxConfig {
    pub fn default() -> Self {
        let config = Self {
            checkbox_base: "flex items-center".to_string(),
            checkbox_container: "relative flex items-center".to_string(),
            checkbox_input: "h-4 w-4 rounded border-gray-300 text-primary-600 focus:ring-primary-500".to_string(),
            checkbox_label: "ml-2 text-sm text-gray-700".to_string(),
            checkbox_checked: "bg-primary-600 border-transparent".to_string(),
            checkbox_unchecked: "bg-white border-gray-300".to_string(),
            checkbox_disabled: "opacity-50 cursor-not-allowed".to_string(),
        };
        config.to_owned()
    }

    pub fn set_checkbox_base(mut self, value: String) -> Self {
        self.checkbox_base = value;
        self
    }

    pub fn set_checkbox_container(mut self, value: String) -> Self {
        self.checkbox_container = value;
        self
    }

    pub fn set_checkbox_input(mut self, value: String) -> Self {
        self.checkbox_input = value;
        self
    }

    pub fn set_checkbox_label(mut self, value: String) -> Self {
        self.checkbox_label = value;
        self
    }

    pub fn set_checkbox_checked(mut self, value: String) -> Self {
        self.checkbox_checked = value;
        self
    }

    pub fn set_checkbox_unchecked(mut self, value: String) -> Self {
        self.checkbox_unchecked = value;
        self
    }

    pub fn set_checkbox_disabled(mut self, value: String) -> Self {
        self.checkbox_disabled = value;
        self
    }
}