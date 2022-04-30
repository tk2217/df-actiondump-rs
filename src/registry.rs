// use std::collections::HashMap;
// use crate::schema::CodeData;
//
// pub struct CodeDatabase {
//     pub values: HashMap<String, CodeRegistry>,
// }
//
// impl CodeDatabase {
//     pub fn new(values: HashMap<String, Vec<CodeData>>) -> Self {
//         let mut internal = HashMap::new();
//
//         for (name, items) in values {
//             internal.insert(name.clone(), CodeRegistry::new(name, items));
//         }
//
//         CodeDatabase {
//             values: internal
//         }
//     }
// }
//
// pub struct CodeRegistry<'a> {
//     pub name: String,
//     pub items: Vec<CodeData>,
//     pub by_name: HashMap<String, &'a CodeData>,
//     pub by_id: HashMap<String, &'a CodeData>,
// }
//
// impl<'a> CodeRegistry<'a> {
//     pub fn new(name: String, items: Vec<CodeData>) -> Self {
//         let mut by_name = HashMap::new();
//         let mut by_id = HashMap::new();
//
//         for item in &items {
//             by_name.insert(item.name.to_owned(), item);
//             match &item.identifier {
//                 Some(identifier) => by_id.insert(identifier.to_owned(), item),
//                 _ => {}
//             }
//
//         }
//
//         CodeRegistry {
//             name,
//             items,
//             by_name,
//             by_id
//         }
//     }
// }