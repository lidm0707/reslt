use dioxus::prelude::*;

/// Skeleton variant for visual style
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SkeletonVariant {
    Default,
    Circular,
    Rounded,
}

/// Base skeleton component with Tailwind CSS
///
/// This component provides a loading placeholder that mimics the structure
/// of the content that will be displayed.
///
/// # Example
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn ProfileSkeleton() -> Element {
///     rsx! {
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
    /// Width of the skeleton (e.g., "100%", "3rem", "200px")
    #[props(default = String::from("100%"))]
    width: String,
    /// Height of the skeleton (e.g., "1rem", "20px")
    #[props(default = String::from("1rem"))]
    height: String,
    /// Visual style variant
    #[props(default = SkeletonVariant::Default)]
    variant: SkeletonVariant,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
    /// Animation variant (pulse, wave, none)
    #[props(default = String::from("pulse"))]
    animation: String,
) -> Element {
    let shape_class = match variant {
        SkeletonVariant::Circular => "rounded-full",
        SkeletonVariant::Rounded => "rounded-lg",
        SkeletonVariant::Default => "rounded",
    };

    let animation_class = if animation == "none" {
        ""
    } else {
        "animate-pulse"
    };

    rsx! {
        div {
            class: "bg-gray-200 {shape_class} {animation_class} {class}",
            style: "width: {width}; height: {height};",
        }
    }
}

/// Text skeleton for simulating text content
///
/// # Example
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn ArticleSkeleton() -> Element {
///     rsx! {
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
            class: "space-y-2 {class}",
            for index in 0..lines {
                Skeleton {
                    width: if index == lines - 1 {
                        widths[index % widths.len() * 2 / 3]
                    } else {
                        widths[index % widths.len()]
                    },
                    height: height.clone(),
                    class: if index == 0 { "w-full" } else { "" },
                }
            }
        }
    }
}

/// Avatar skeleton for user avatars
///
/// # Example
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn UserListSkeleton() -> Element {
///     rsx! {
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
        Skeleton {
            width: size.clone(),
            height: size,
            variant: SkeletonVariant::Circular,
            class: "flex-shrink-0 {class}",
        }
    }
}

/// Image skeleton for content images
///
/// # Example
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn GallerySkeleton() -> Element {
///     rsx! {
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
    #[props(default = String::from("rounded-lg"))]
    radius: String,
    /// Additional CSS classes
    #[props(default = String::new())]
    class: String,
) -> Element {
    rsx! {
        Skeleton {
            width,
            height,
            variant: if radius == "rounded-full" {
                SkeletonVariant::Circular
            } else {
                SkeletonVariant::Default
            },
            class: "{class} {radius}",
        }
    }
}

/// Button skeleton for buttons
///
/// # Example
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn ButtonRowSkeleton() -> Element {
///     rsx! {
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
        Skeleton {
            width,
            height,
            variant: SkeletonVariant::Rounded,
            class: "{class}",
        }
    }
}

/// Card skeleton for card layouts
///
/// # Example
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn CardGridSkeleton() -> Element {
///     rsx! {
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
    /// Whether to show image section
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
            class: "bg-white rounded-lg shadow-md overflow-hidden {class}",
            if show_image {
                ImageSkeleton {
                    width: "100%".to_string(),
                    height: image_height,
                    radius: "rounded-t-lg".to_string(),
                }
            }
            div { class: "p-4",
                TextSkeleton {
                    lines: title_lines,
                    height: "1.25rem".to_string(),
                    class: "mb-3",
                }
                TextSkeleton {
                    lines: description_lines,
                    height: "0.875rem".to_string(),
                }
            }
        }
    }
}

/// Table row skeleton for table loading state
///
/// # Example
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn TableSkeleton() -> Element {
///     rsx! {
///         Table {
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
            class: "border border-gray-200 rounded-lg overflow-hidden {class}",
            // Header
            div {
                class: "bg-gray-50 border-b border-gray-200 flex divide-x divide-gray-200",
                for _ in 0..columns {
                    div {
                        class: "flex-1 flex items-center px-6",
                        Skeleton {
                            width: "60%".to_string(),
                            height: header_height.clone(),
                            class: "h-0",
                        }
                    }
                }
            }
            // Rows
            for _ in 0..rows {
                div {
                    class: "border-b border-gray-200 flex divide-x divide-gray-200",
                    for _ in 0..columns {
                        div {
                            class: "flex-1 flex items-center px-6",
                            Skeleton {
                                width: format!("{}%", 30 + (rand::random::<u32>() % 50)),
                                height: cell_height.clone(),
                                class: "h-0",
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
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn CommentListSkeleton() -> Element {
///     rsx! {
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
            class: "space-y-4 {class}",
            for _ in 0..items {
                div {
                    class: "flex items-start space-x-4 p-4 bg-white rounded-lg",
                    if show_avatar {
                        AvatarSkeleton { size: "3rem".to_string() }
                    }
                    div { class: "flex-1 space-y-2",
                        Skeleton {
                            width: "70%".to_string(),
                            height: "1rem".to_string(),
                        }
                        TextSkeleton {
                            lines: content_lines,
                            height: "0.875rem".to_string(),
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
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::skeleton::*;
///
/// #[component]
/// fn FormSkeleton() -> Element {
///     rsx! {
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
                Skeleton {
                    width: "30%".to_string(),
                    height: "0.875rem".to_string(),
                    class: "mb-2",
                }
            }
            Skeleton {
                width,
                height,
                variant: SkeletonVariant::Rounded,
            }
        }
    }
}
