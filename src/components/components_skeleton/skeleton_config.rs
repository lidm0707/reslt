use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Props)]
pub struct SkeletonConfig {
    pub skeleton_base: String,
    pub skeleton_text: String,
    pub skeleton_rectangular: String,
    pub skeleton_rounded: String,
    pub skeleton_circular: String,
    pub skeleton_animate: String,
}

impl SkeletonConfig {
    pub fn default() -> Self {
        let config = Self {
            skeleton_base: r#"background-color: #d1d5db; border-radius: 0.25rem;"#.to_string(),
            skeleton_text: r#"background-color: #d1d5db; height: 1rem; width: 100%; border-radius: 0.25rem;"#.to_string(),
            skeleton_rectangular: r#"background-color: #d1d5db;"#.to_string(),
            skeleton_rounded: r#"background-color: #d1d5db; border-radius: 0.375rem;"#.to_string(),
            skeleton_circular: r#"background-color: #d1d5db; border-radius: 9999px;"#.to_string(),
            skeleton_animate: r#"background-color: #d1d5db; animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;"#.to_string(),
        };
        config.to_owned()
    }

    pub fn set_skeleton_base(mut self, value: String) -> Self {
        self.skeleton_base = value;
        self
    }

    pub fn set_skeleton_text(mut self, value: String) -> Self {
        self.skeleton_text = value;
        self
    }

    pub fn set_skeleton_rectangular(mut self, value: String) -> Self {
        self.skeleton_rectangular = value;
        self
    }

    pub fn set_skeleton_rounded(mut self, value: String) -> Self {
        self.skeleton_rounded = value;
        self
    }

    pub fn set_skeleton_circular(mut self, value: String) -> Self {
        self.skeleton_circular = value;
        self
    }

    pub fn set_skeleton_animate(mut self, value: String) -> Self {
        self.skeleton_animate = value;
        self
    }
}
