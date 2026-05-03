pub trait FieldAccessible {
    fn get_field(&self, field_name: &str) -> Option<String>;
}
