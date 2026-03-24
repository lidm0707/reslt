pub mod app;
pub mod home;

// Re-export components from root for easier importing
pub use crate::components::{create_person_columns, DefaultTable, ModalContainer, ToastContainer};

// Re-export app component
pub use app::App;
