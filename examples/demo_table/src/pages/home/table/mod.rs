mod hard_data;
mod modal;
mod service;
pub mod table_data;

// Re-export types and functions for easier importing
pub use hard_data::get_hardcoded_data;
pub use modal::ModalCreate;
pub use service::{create_person, delete_rows, get_person_data, update_person};
pub use table_data::Person;
