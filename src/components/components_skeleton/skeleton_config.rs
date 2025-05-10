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
            skeleton_base: "skeleton bg-gray-300 dark:bg-gray-700 rounded".to_string(),
            skeleton_text: "skeleton-text h-4 w-full rounded".to_string(),
            skeleton_rectangular: "skeleton-rectangular".to_string(),
            skeleton_rounded: "skeleton-rounded rounded-md".to_string(),
            skeleton_circular: "skeleton-circular rounded-full".to_string(),
            skeleton_animate: "skeleton-wave animate-pulse".to_string(),
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
