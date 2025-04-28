// use syn::Attribute;

// pub fn parse_field_attributes(attrs: &[Attribute]) -> (String, String, Option<String>, Option<String>) {
//     let mut head = String::new();
//     let mut index = String::new();
//     let mut class = None;
//     let mut action = None;

//     for attr in attrs {
//         if attr.path().is_ident("field") {
//             let _ = attr.parse_nested_meta(|meta| {
//                 if meta.path.is_ident("head") {
//                     let value = meta.value()?;
//                     let str_lit: syn::LitStr = value.parse()?;
//                     head = str_lit.value();
//                 } else if meta.path.is_ident("index") {
//                     let value = meta.value()?;
//                     let str_lit: syn::LitStr = value.parse()?;
//                     index = str_lit.value();
//                 } else if meta.path.is_ident("class") {
//                     let value = meta.value()?;
//                     let str_lit: syn::LitStr = value.parse()?;
//                     class = Some(str_lit.value());
//                 } else if meta.path.is_ident("action") {
//                     let value = meta.value()?;
//                     let str_lit: syn::LitStr = value.parse()?;
//                     action = Some(str_lit.value());
//                 }
//                 Ok(())
//             });
//         }
//     }

//     (head, index, class, action)
// }