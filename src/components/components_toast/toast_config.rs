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
            toast_container: "fixed top-0 right-0 p-4 z-50 space-y-4 w-80".to_string(),
            toast_item: "flex items-center p-4 mb-4 rounded-lg shadow-md".to_string(),
            toast_success: "text-green-800 bg-green-100".to_string(),
            toast_error: "text-red-800 bg-red-100".to_string(),
            toast_warning: "text-yellow-800 bg-yellow-100".to_string(),
            toast_info: "text-blue-800 bg-blue-100".to_string(),
            toast_icon: "inline-flex items-center justify-center flex-shrink-0 w-8 h-8 mr-3 rounded-lg".to_string(),
            toast_message: "text-sm font-normal".to_string(),
            toast_close_button: "ml-auto -mx-1.5 -my-1.5 rounded-lg focus:ring-2 p-1.5 inline-flex h-8 w-8 hover:bg-gray-200".to_string(),
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