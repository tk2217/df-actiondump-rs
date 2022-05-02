use crate::schema::CodeData;
use std::collections::HashMap;

pub type DatabaseValues = HashMap<String, Vec<CodeData>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeDatabase {
    pub values: HashMap<String, CodeRegistry>,
}

impl CodeDatabase {
    pub fn new(values: DatabaseValues) -> Self {
        let mut internal = HashMap::new();

        for (k, v) in values.into_iter() {
            internal.insert(k.to_owned(), CodeRegistry::new(k, v));
        }

        CodeDatabase { values: internal }
    }

    pub fn into_values(self) -> DatabaseValues {
        self.values
            .into_iter()
            .map(|(name, registry)| (name, registry.items))
            .collect()
    }
}

/// A registry to hold a set of [`CodeData`]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeRegistry {
    /// The name of this [`CodeRegistry`].
    pub name: String,
    /// A list of all the items in this registry.
    pub items: Vec<CodeData>,
    /// A mapping from name to index for all [`CodeData`] elements where it is present.
    by_name: HashMap<String, usize>,
    /// A mapping from identifier to index for all [`CodeData`] elements where it is present.
    by_id: HashMap<String, usize>,
}

impl CodeRegistry {
    pub fn new(name: String, items: Vec<CodeData>) -> Self {
        let by_name = items
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.name.to_owned().map(|k| (k, i)))
            .collect();
        let by_id = items
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.identifier.to_owned().map(|k| (k, i)))
            .collect();

        CodeRegistry {
            name,
            items,
            by_name,
            by_id,
        }
    }

    pub fn get_by_name(&self, id: String) -> Option<&CodeData> {
        self.items.get(self.by_id.get(&id)?.to_owned())
    }

    pub fn get_by_id(&self, id: String) -> Option<&CodeData> {
        self.items.get(self.by_id.get(&id)?.to_owned())
    }
}
