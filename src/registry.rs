//! An interface for loading and accessing a full action dump.

use crate::schema::CodeData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type DatabaseValues = HashMap<String, CodeRegistry>;


/// A full code database which can be used to load an action dump.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodeDatabase {
    /// The code database values.
    #[serde(flatten)]
    pub values: DatabaseValues,
}

impl CodeDatabase {
    /// Create a new code database from its values.
    pub fn new(values: DatabaseValues) -> Self {
        CodeDatabase {
            values
        }
    }
}

impl Into<DatabaseValues> for CodeDatabase {
    fn into(self) -> DatabaseValues {
        self.values
    }
}

/// A registry holding a [`Vec<CodeData>`] and indexes from names and values.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(from = "Vec<CodeData>", into = "Vec<CodeData>")]
pub struct CodeRegistry {
    /// A list of all the items in this registry.
    pub items: Vec<CodeData>,
    /// A mapping from name to index for all [`CodeData`] elements where it is present.
    by_name: HashMap<String, usize>,
    /// A mapping from identifier to index for all [`CodeData`] elements where it is present.
    by_id: HashMap<String, usize>,
}

impl CodeRegistry {
    /// Create a [`CodeRegistry`] from a [`Vec<CodeData>`].
    ///
    /// This will iterate over the list of items at least 2 times.
    pub fn new(items: Vec<CodeData>) -> Self {
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
            items,
            by_name,
            by_id,
        }
    }

    /// Fetch a [`CodeData`] element by its `name`.
    pub fn get_by_name(&self, name: String) -> Option<&CodeData> {
        self.items.get(self.by_name.get(&name)?.to_owned())
    }

    /// Fetch a [`CodeData`] element by its `identifier`.
    pub fn get_by_id(&self, id: String) -> Option<&CodeData> {
        self.items.get(self.by_id.get(&id)?.to_owned())
    }
}

impl From<Vec<CodeData>> for CodeRegistry {
    fn from(internal: Vec<CodeData>) -> Self {
        CodeRegistry::new(internal)
    }
}

impl Into<Vec<CodeData>> for CodeRegistry {
    fn into(self) -> Vec<CodeData> {
        self.items
    }
}
