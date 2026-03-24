use dioxus::prelude::*;
use reslt_core::prelude::*;

#[component]
pub fn ModalContainer() -> Element {
    let modal = use_context::<UseModal>().to_owned();

    let is_open = modal.is_open();
    let title = modal.get_title();
    let content = modal.get_content();

    // Clone modal for use in closures to avoid move errors
    let modal_close = modal.clone();

    rsx! {
        if is_open {
            div {
                class: "fixed inset-0 z-50 overflow-y-auto",
                "aria-labelledby": "modal-title",
                "role": "dialog",
                "aria-modal": "true",

                // Backdrop
                div {
                    class: "flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0",

                    // Background overlay
                    div {
                        class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity",
                        "aria-hidden": "true",
                        onclick: move |_| {
                            modal_close.close();
                        }
                    }

                    // Center alignment helper
                    span {
                        class: "hidden sm:inline-block sm:align-middle sm:h-screen",
                        "aria-hidden": "true",
                        "​"
                    }

                    // Modal panel
                    div {
                        class: "inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",

                        // Modal header
                        div {
                            class: "bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                            div {
                                class: "sm:flex sm:items-start",
                                div {
                                    class: "mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-blue-100 sm:mx-0 sm:h-10 sm:w-10",
                                    svg {
                                        class: "h-6 w-6 text-blue-600",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "stroke-width": "2",
                                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                        }
                                    }
                                }
                                div {
                                    class: "mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left",
                                    h3 {
                                        class: "text-lg leading-6 font-medium text-gray-900",
                                        id: "modal-title",
                                        "{title}"
                                    }
                                }
                            }
                        }

                        // Modal body
                        div {
                            class: "px-4 py-3 sm:px-6",
                            {content}
                        }

                        // Modal footer
                        div {
                            class: "bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse",
                            button {
                                class: "w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-blue-600 text-base font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 sm:ml-3 sm:w-auto sm:text-sm",
                                onclick: move |_| {
                                    modal_close.close();
                                },
                                "Close"
                            }
                        }
                    }
                }
            }
        }
    }
}
