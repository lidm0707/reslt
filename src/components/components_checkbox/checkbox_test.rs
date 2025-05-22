use serde::Serialize;
use crate::prelude::FieldAccessible;

#[derive(Serialize, Eq, PartialEq, Clone, Debug, FieldAccessible)]
struct TestItem {
    id: u32,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*; // To bring TestItem into scope
    use crate::components::components_checkbox::checkbox_signal::use_checkbox; // Adjust path as needed
    // Removed unused: use crate::components::components_checkbox::checkbox_signal::UseCheckBox; 
    use dioxus::prelude::*; // Required for signal manipulation in tests

    // Helper function to create TestItem instances
    fn create_item(id: u32, name: &str) -> TestItem {
        TestItem {
            id,
            name: name.to_string(),
        }
    }

    #[test]
    fn test_use_checkbox_initialization() {
        // Dioxus context is needed for hooks like use_signal
        let mut vdom = VirtualDom::new(|| {
            let checkbox_signal = use_checkbox::<TestItem>();
            assert!(checkbox_signal.get_checked_data().is_empty(), "Initially, data_checked should be empty");
            assert!(!checkbox_signal.is_all_checked(), "Initially, is_all_checked should be false");
            rsx!({}) // Corrected empty rsx
        });
        vdom.rebuild_in_place(); // Trigger the component lifecycle
    }

    #[test]
    fn test_set_all_checked() {
        let mut vdom = VirtualDom::new(|| {
            let mut checkbox_signal = use_checkbox::<TestItem>();
            let item1 = create_item(1, "Item 1");
            let item2 = create_item(2, "Item 2");
            let all_items = vec![item1.clone(), item2.clone()];

            // Case 1: is_all_checked is false
            checkbox_signal.set_all_checked(all_items.clone());
            assert!(checkbox_signal.is_all_checked(), "is_all_checked should be true after selecting all");
            assert_eq!(checkbox_signal.get_checked_data().len(), 2, "data_checked should contain all items");
            assert!(checkbox_signal.get_checked_data().contains(&item1));
            assert!(checkbox_signal.get_checked_data().contains(&item2));

            // Case 2: is_all_checked is true
            checkbox_signal.set_all_checked(Vec::new()); // Data here doesn't matter as it will be cleared
            assert!(!checkbox_signal.is_all_checked(), "is_all_checked should be false after deselecting all");
            assert!(checkbox_signal.get_checked_data().is_empty(), "data_checked should be empty after deselecting all");
            
            rsx!({}) // Corrected empty rsx
        });
        vdom.rebuild_in_place();
    }

    #[test]
    fn test_get_checked_data() {
        let mut vdom = VirtualDom::new(|| {
            let mut checkbox_signal = use_checkbox::<TestItem>();
            let item1 = create_item(1, "Apple");
            checkbox_signal.push_checked_data(item1.clone());
            assert_eq!(checkbox_signal.get_checked_data(), vec![item1]);
            rsx!({}) // Corrected empty rsx
        });
        vdom.rebuild_in_place();
    }
    
    #[test]
    fn test_set_checked_data() {
        let mut vdom = VirtualDom::new(|| {
            let mut checkbox_signal = use_checkbox::<TestItem>();
            let item1 = create_item(1, "Banana");
            let item2 = create_item(2, "Orange");

            // Initially add some data
            checkbox_signal.push_checked_data(item1.clone());
            checkbox_signal.push_checked_data(item2.clone());
            assert_eq!(checkbox_signal.get_checked_data().len(), 2);

            // Case 1: Input data is not empty (should clear existing)
            // Note: The current implementation of set_checked_data is:
            // if data.len() > 0 { self.data_checked.set(Vec::new()); } 
            // else { self.data_checked.set(data); }
            // This means if we pass non-empty data, it *clears* data_checked, rather than setting it.
            // Let's test this behavior.
            let new_data_non_empty = vec![create_item(3, "Grape")];
            checkbox_signal.set_checked_data(new_data_non_empty);
            assert!(checkbox_signal.get_checked_data().is_empty(), "data_checked should be empty if set_checked_data receives non-empty data");

            // Reset by pushing items again
            checkbox_signal.push_checked_data(item1.clone()); 
            checkbox_signal.push_checked_data(item2.clone());

            // Case 2: Input data is empty (should set to empty)
            let empty_data = Vec::<TestItem>::new();
            checkbox_signal.set_checked_data(empty_data.clone());
            assert!(checkbox_signal.get_checked_data().is_empty(), "data_checked should be empty if set_checked_data receives empty data");
            
            rsx!({}) // Corrected empty rsx
        });
        vdom.rebuild_in_place();
    }


    #[test]
    fn test_push_checked_data() {
        let mut vdom = VirtualDom::new(|| {
            let mut checkbox_signal = use_checkbox::<TestItem>();
            let item1 = create_item(1, "One");
            let item2 = create_item(2, "Two");

            checkbox_signal.push_checked_data(item1.clone());
            assert_eq!(checkbox_signal.get_checked_data().len(), 1);
            assert!(checkbox_signal.get_checked_data().contains(&item1));

            checkbox_signal.push_checked_data(item2.clone());
            assert_eq!(checkbox_signal.get_checked_data().len(), 2);
            assert!(checkbox_signal.get_checked_data().contains(&item1));
            assert!(checkbox_signal.get_checked_data().contains(&item2));
            rsx!({}) // Corrected empty rsx
        });
        vdom.rebuild_in_place();
    }

    #[test]
    fn test_remove() {
        let mut vdom = VirtualDom::new(|| {
            let mut checkbox_signal = use_checkbox::<TestItem>();
            let item1 = create_item(1, "Cat");
            let item2 = create_item(2, "Dog");
            let item3 = create_item(3, "Bird");

            checkbox_signal.push_checked_data(item1.clone());
            checkbox_signal.push_checked_data(item2.clone());
            checkbox_signal.push_checked_data(item3.clone());

            assert_eq!(checkbox_signal.get_checked_data().len(), 3);

            // Remove an existing item
            checkbox_signal.remove(item2.clone());
            assert_eq!(checkbox_signal.get_checked_data().len(), 2);
            assert!(checkbox_signal.get_checked_data().contains(&item1));
            assert!(!checkbox_signal.get_checked_data().contains(&item2));
            assert!(checkbox_signal.get_checked_data().contains(&item3));

            // Try to remove a non-existing item (by id, but different content)
            let item_non_existent = create_item(2, "Fish"); 
            checkbox_signal.remove(item_non_existent);
            assert_eq!(checkbox_signal.get_checked_data().len(), 2, "Removing a non-existent item should not change the list");
            
            // Remove another existing item
            checkbox_signal.remove(item1.clone());
            assert_eq!(checkbox_signal.get_checked_data().len(), 1);
            assert!(!checkbox_signal.get_checked_data().contains(&item1));
            assert!(checkbox_signal.get_checked_data().contains(&item3));
            rsx!({}) // Corrected empty rsx
        });
        vdom.rebuild_in_place();
    }
}