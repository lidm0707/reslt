// #[component]
// pub fn CheckBox<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(table: UseTable<T>) -> Element{
//             rsx! {
//                 input {
//                     r#type: "checkbox",
//                     checked: *is_all_checked.get(),
//                     onchange: move |evt| {
//                         let check_all = evt.value();
//                         is_all_checked.set(check_all);
//                         if check_all {
//                             let all_rows: Vec<T> = table.get_all_rows().to_vec();
//                             table.set_checked_rows(all_rows);
//                         } else {
//                             table.clear_checked_rows();
//                         }
//                     }
//                 }
//             }
// }
