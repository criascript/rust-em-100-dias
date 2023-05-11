use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// A single item in the database.
pub struct Item {
    /// The name of the item.
    pub name: String,
    /// The price of the item.
    pub price: f32,
    /// The quantity of the item.
    pub quantity: u32,
}

#[derive(Debug, Default, Clone)]
/// A simple in-memory database that store [Item]s.
pub struct Db {
    inner: Arc<RwLock<HashMap<String, Item>>>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    /// Insert a new item into the database.
    pub fn insert(&self, item: Item) {
        self.inner.write().insert(item.name.clone(), item);
    }

    /// Get an item from the database.
    pub fn get(&self, name: &str) -> Option<Item> {
        self.inner.read().get(name).cloned()
    }

    /// List all items in the database.
    pub fn list(&self) -> Vec<Item> {
        self.inner.read().values().cloned().collect()
    }

    /// Update an item in the database.
    pub fn update(&self, item: Item) {
        self.insert(item);
    }

    /// Delete an item from the database.
    pub fn delete(&self, name: &str) {
        self.inner.write().remove(name);
    }
}