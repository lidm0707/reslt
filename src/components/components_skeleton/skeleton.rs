use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Skeleton(#[props(default = SkeletonConfig::default())] config: SkeletonConfig) -> Element {
    let class_string = [
        config.skeleton_base,
        config.skeleton_text,
        config.skeleton_rectangular,
        config.skeleton_rounded,
        config.skeleton_circular,
        config.skeleton_animate,
    ]
    .join(" ");

    rsx! {
        div {
            class: format!("{}", class_string),
        }
    }
}
