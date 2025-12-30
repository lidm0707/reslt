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
            checkbox_base: r#"display: flex; align-items: center;"#.to_string(),
            checkbox_container: r#"position: absolute; display: flex; align-items: center;;"#
                .to_string(),
            checkbox_input:
                r#"height: 1rem; width: 1rem; border-radius: 0.25rem; border: 1px solid #d1d5db;"#
                    .to_string(),
            checkbox_label: r#"margin-left: 0.5rem; font-size: 0.875rem; color: #374151;"#
                .to_string(),
            checkbox_checked: r#"background-color: #2563eb; border-color: transparent;;"#
                .to_string(),
            checkbox_unchecked: r#"background-color: white; border-color: #d1d5db; ;"#.to_string(),
            checkbox_disabled: r#"opacity: 0.5; cursor: not-allowed;"#.to_string(),
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
