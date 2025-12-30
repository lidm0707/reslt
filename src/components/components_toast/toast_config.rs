use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Props)]
pub struct ToastConfig {
    pub toast_container: String,
    pub toast_item: String,
    pub toast_success: String,
    pub toast_error: String,
    pub toast_warning: String,
    pub toast_info: String,
    pub toast_icon: String,
    pub toast_message: String,
    pub toast_close_button: String,
}

impl ToastConfig {
    pub fn default() -> Self {
        let config = Self {
            toast_container: r#"position: fixed; top: 0; right: 0; padding: 1rem; z-index: 50; width: 20rem;"#.to_string(),
            toast_item: r#"display: flex; align-items: center; padding: 1rem; margin-bottom: 1rem; border-radius: 0.5rem; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);"#.to_string(),
            toast_success: r#"color: #166534; background-color: #dcfce7;"#.to_string(),
            toast_error: r#"color: #991b1b; background-color: #fee2e2;"#.to_string(),
            toast_warning: r#"color: #92400e; background-color: #fef3c7;"#.to_string(),
            toast_info: r#"color: #1e40af; background-color: #dbeafe;"#.to_string(),
            toast_icon: r#"display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0; width: 2rem; height: 2rem; margin-right: 0.75rem; border-radius: 0.5rem;"#.to_string(),
            toast_message: r#"font-size: 0.875rem; font-weight: normal;"#.to_string(),
            toast_close_button: r#"margin-left: auto; margin-left: -0.375rem; margin-top: -0.375rem; border-radius: 0.5rem; padding: 0.375rem; display: inline-flex; height: 2rem; width: 2rem;"#.to_string(),
        };
        config.to_owned()
    }

    pub fn set_toast_container(mut self, value: String) -> Self {
        self.toast_container = value;
        self
    }

    pub fn set_toast_item(mut self, value: String) -> Self {
        self.toast_item = value;
        self
    }

    pub fn set_toast_success(mut self, value: String) -> Self {
        self.toast_success = value;
        self
    }

    pub fn set_toast_error(mut self, value: String) -> Self {
        self.toast_error = value;
        self
    }

    pub fn set_toast_warning(mut self, value: String) -> Self {
        self.toast_warning = value;
        self
    }

    pub fn set_toast_info(mut self, value: String) -> Self {
        self.toast_info = value;
        self
    }

    pub fn set_toast_icon(mut self, value: String) -> Self {
        self.toast_icon = value;
        self
    }

    pub fn set_toast_message(mut self, value: String) -> Self {
        self.toast_message = value;
        self
    }

    pub fn set_toast_close_button(mut self, value: String) -> Self {
        self.toast_close_button = value;
        self
    }
}
