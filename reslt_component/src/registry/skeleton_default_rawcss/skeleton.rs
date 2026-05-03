use dioxus::prelude::*;

/// Skeleton variant for visual style
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SkeletonVariant {
    Default,
    Circular,
    Rounded,
}

/// Base skeleton component with raw CSS
///
/// This component provides a loading placeholder that mimics the structure
/// of the content that will be displayed.
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn ProfileSkeleton() -> Element {
///     rsx! {
///         SkeletonStylesheet {}
///
///         div { class: "flex items-center space-x-4",
///             Skeleton {
///                 width: "3rem",
///                 height: "3rem",
///                 variant: SkeletonVariant::Circular,
///                 class: "flex-shrink-0",
///             }
///             div { class: "flex-1 space-y-2",
///                 Skeleton { width: "60%", height: "1rem" }
///                 Skeleton { width: "40%", height: "0.875rem" }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn Skeleton(
    /// Width of skeleton (e.g., "100%", "3rem", "200px")
    #[props(default = String::from("100%"))]
    width: String,
    /// Height of skeleton (e.g., "1rem", "20px")
    #[props(default = String::from("1rem"))]
    height: String,
    /// Visual style variant
    #[props(default = SkeletonVariant::Default)]
    variant: SkeletonVariant,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
    /// Animation variant (pulse, none)
    #[props(default = String::from("pulse"))]
    animation: String,
) -> Element {
    let shape_class = match variant {
        SkeletonVariant::Circular => "reslt-skeleton-circular",
        SkeletonVariant::Rounded => "reslt-skeleton-rounded",
        SkeletonVariant::Default => "reslt-skeleton-default",
    };

    let animation_class = if animation == "none" {
        "reslt-skeleton-no-animation"
    } else {
        ""
    };

    rsx! {
        div {
            class: "reslt-skeleton {shape_class} {animation_class} {class}",
            style: "width: {width}; height: {height};",
        }
    }
}

/// Text skeleton for simulating text content
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn ArticleSkeleton() -> Element {
///     rsx! {
///         SkeletonStylesheet {}
///
///         div { class: "space-y-4",
///             TextSkeleton { lines: 3, height: "1rem" }
///             TextSkeleton { lines: 2, height: "0.875rem" }
///         }
///     }
/// }
/// ```
#[component]
pub fn TextSkeleton(
    /// Number of lines to render
    #[props(default = 1)]
    lines: usize,
    /// Height of each line
    #[props(default = String::from("1rem"))]
    height: String,
    /// Whether to randomly vary line widths
    #[props(default = true)]
    random_width: bool,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    let widths = if random_width {
        vec!["100%", "90%", "80%", "70%", "60%", "50%", "40%", "30%"]
    } else {
        vec!["100%"]
    };

    rsx! {
        div {
            class: "reslt-text-skeleton {class}",
            for index in 0..lines {
                div {
                    class: "reslt-text-skeleton-line",
                    style: format!("width: {}; height: {};",
                        if index == lines - 1 {
                            widths[(index * 2 / 3) % widths.len()]
                        } else {
                            widths[index % widths.len()]
                        },
                        height)
                }
            }
        }
    }
}

/// Avatar skeleton for user avatars
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn UserListSkeleton() -> Element {
///     rsx! {
///         SkeletonStylesheet {}
///
///         div { class: "flex items-center space-x-4",
///             for _ in 0..5 {
///                 AvatarSkeleton { size: "3rem" }
///                 div { class: "flex-1 space-y-2",
///                     Skeleton { width: "60%", height: "1rem" }
///                     Skeleton { width: "40%", height: "0.75rem" }
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn AvatarSkeleton(
    /// Size of the avatar (e.g., "3rem", "48px")
    #[props(default = String::from("3rem"))]
    size: String,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "reslt-avatar-skeleton {class}",
            style: "width: {size}; height: {size};",
        }
    }
}

/// Image skeleton for content images
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn GallerySkeleton() -> Element {
///     rsx! {
///         SkeletonStylesheet {}
///
///         div { class: "grid grid-cols-3 gap-4",
///             for _ in 0..6 {
///                 ImageSkeleton {
///                     width: "100%",
///                     height: "12rem",
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ImageSkeleton(
    /// Width of the image
    #[props(default = String::from("100%"))]
    width: String,
    /// Height of the image
    #[props(default = String::from("12rem"))]
    height: String,
    /// Border radius variant
    #[props(default = String::from("rounded"))]
    radius: String,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    let radius_class = match radius.as_str() {
        "rounded-full" => "reslt-image-skeleton-circular",
        "rounded-lg" | "rounded" => "reslt-image-skeleton-rounded",
        _ => "",
    };

    rsx! {
        div {
            class: "reslt-image-skeleton {radius_class} {class}",
            style: "width: {width}; height: {height};",
        }
    }
}

/// Button skeleton for buttons
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn ButtonRowSkeleton() -> Element {
///     rsx! {
///         SkeletonStylesheet {}
///
///         div { class: "flex space-x-4",
///             ButtonSkeleton { width: "6rem" }
///             ButtonSkeleton { width: "8rem" }
///             ButtonSkeleton { width: "5rem" }
///         }
///     }
/// }
/// ```
#[component]
pub fn ButtonSkeleton(
    /// Width of the button
    #[props(default = String::from("6rem"))]
    width: String,
    /// Height of the button
    #[props(default = String::from("2.5rem"))]
    height: String,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "reslt-button-skeleton {class}",
            style: "width: {width}; height: {height};",
        }
    }
}

/// Card skeleton for card layouts
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn CardGridSkeleton() -> Element {
///     rsx! {
///         SkeletonStylesheet {}
///
///         div { class: "grid grid-cols-3 gap-6",
///             for _ in 0..6 {
///                 CardSkeleton {}
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn CardSkeleton(
    /// Whether to show the image section
    #[props(default = true)]
    show_image: bool,
    /// Height of the image section
    #[props(default = String::from("12rem"))]
    image_height: String,
    /// Number of title lines
    #[props(default = 2)]
    title_lines: usize,
    /// Number of description lines
    #[props(default = 3)]
    description_lines: usize,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "reslt-card-skeleton {class}",
            if show_image {
                div {
                    class: "reslt-card-skeleton-image reslt-card-skeleton-image-top",
                    style: "height: {image_height};",
                }
            }
            div {
                class: "reslt-card-skeleton-content",
                for _ in 0..title_lines {
                    div {
                        class: "reslt-card-skeleton-title",
                        style: "width: 70%;",
                    }
                }
                div {
                    class: "reslt-card-skeleton-description",
                    for _ in 0..description_lines {
                        div {
                            class: "reslt-card-skeleton-description-line",
                            style: format!("width: {}%;", 40 + ((rand::random::<u32>() % 40) as i32)),
                        }
                    }
                }
            }
        }
    }
}

/// Table row skeleton for table loading state
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn TableSkeleton() -> Element {
///     rsx! {
///         SkeletonStylesheet {}
///
///         TableSkeleton {
///             columns: 5,
///             rows: 5,
///         }
///     }
/// }
/// ```
#[component]
pub fn TableSkeleton(
    /// Number of columns
    #[props(default = 5)]
    columns: usize,
    /// Number of rows
    #[props(default = 5)]
    rows: usize,
    /// Header height
    #[props(default = String::from("3rem"))]
    header_height: String,
    /// Cell height
    #[props(default = String::from("3rem"))]
    cell_height: String,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "reslt-table-skeleton {class}",
            // Header
            div {
                class: "reslt-table-skeleton-header",
                for _ in 0..columns {
                    div {
                        class: "reslt-table-skeleton-header-cell",
                        div {
                            class: "reslt-table-skeleton-header-skeleton",
                            style: "height: {header_height}; width: 60%;",
                        }
                    }
                }
            }
            // Rows
            for _ in 0..rows {
                div {
                    class: "reslt-table-skeleton-row",
                    for _ in 0..columns {
                        div {
                            class: "reslt-table-skeleton-cell",
                            div {
                                class: "reslt-table-skeleton-cell-skeleton",
                                style: format!("height: {cell_height}; width: {}%;",
                                    30 + ((rand::random::<u32>() % 50) as i32))
                            }
                        }
                    }
                }
            }
        }
    }
}

/// List skeleton for list items
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn CommentListSkeleton() -> Element {
///     rsx! {
///         SkeletonStylesheet {}
///
///         ListSkeleton {
///             items: 5,
///             show_avatar: true,
///             content_lines: 2,
///         }
///     }
/// }
/// ```
#[component]
pub fn ListSkeleton(
    /// Number of list items
    #[props(default = 5)]
    items: usize,
    /// Whether to show avatar for each item
    #[props(default = true)]
    show_avatar: bool,
    /// Number of content lines per item
    #[props(default = 2)]
    content_lines: usize,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "reslt-list-skeleton {class}",
            for _ in 0..items {
                div {
                    class: "reslt-list-skeleton-item",
                    if show_avatar {
                        div {
                            class: "reslt-list-skeleton-avatar",
                            AvatarSkeleton { size: "3rem".to_string() }
                        }
                    }
                    div {
                        class: "reslt-list-skeleton-content",
                        div {
                            class: "reslt-list-skeleton-content-title",
                            style: "width: 70%;",
                        }
                        div {
                            class: "reslt-list-skeleton-content-text",
                            for _ in 0..content_lines {
                                div {
                                    class: "reslt-list-skeleton-content-text-line",
                                    style: format!("width: {}%;", 50 + ((rand::random::<u32>() % 40) as i32)),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Input skeleton for form inputs
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn FormSkeleton() -> Element {
///     rsx! {
///         SkeletonStylesheet {}
///
///         div { class: "space-y-4",
///             InputSkeleton { label: true }
///             InputSkeleton { label: true }
///             InputSkeleton { label: true }
///             ButtonSkeleton { width: "8rem" }
///         }
///     }
/// }
/// ```
#[component]
pub fn InputSkeleton(
    /// Whether to show a label above the input
    #[props(default = false)]
    label: bool,
    /// Width of the input
    #[props(default = String::from("100%"))]
    width: String,
    /// Height of the input
    #[props(default = String::from("2.5rem"))]
    height: String,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "{class}",
            if label {
                div {
                    class: "reslt-input-skeleton-label",
                    style: "width: 30%;",
                }
            }
            div {
                class: "reslt-input-skeleton",
                style: "width: {width}; height: {height};",
            }
        }
    }
}

/// Import skeleton stylesheet
/// Add this component to your app's root to include the skeleton styles
#[component]
pub fn SkeletonStylesheet() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./skeleton.css") }
    }
}
