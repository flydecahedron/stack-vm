//! A key/value table using strings as keys.

use std::collections::HashMap;
use table::Table;

/// A table which allows values to be overwritten.
/// Useful for your language's local variables, etc.
///
/// ```
/// use stack_vm::{MutableTable, Table};
/// let mut table: MutableTable<usize> = MutableTable::new();
/// assert!(table.is_empty());
///
/// table.insert("example", 13);
/// assert!(!table.is_empty());
///
/// assert!(table.contains_key("example"));
///
/// let value = *table.get("example").unwrap();
/// assert_eq!(value, 13);
///
/// table.insert("example", 14);
/// assert!(table.contains_key("example"));
///
/// let value = *table.get("example").unwrap();
/// assert_eq!(value, 14);
/// ```
#[derive(Debug)]
pub struct MutableTable<T>(HashMap<String, T>);

impl<T> MutableTable<T> {
    /// Return a new, empty `MutableTable`.
    pub fn new() ->MutableTable<T> {
        MutableTable(HashMap::new())
    }
}

impl<T> Table for MutableTable<T> {
    type Item = T;

    fn insert(&mut self, name: &str, value: T) {
        let name = String::from(name);
        self.0.insert(name, value);
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn contains_key(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    fn get(&self, name: &str) -> Option<&T> {
        self.0.get(name)
    }
}
#[cfg(test)]
mod test {
    use table::Table;
    use super::*;

    #[test]
    fn new() {
        let mutable_table: MutableTable<usize> = MutableTable::new();
        assert!(mutable_table.is_empty())
    }

    #[test]
    fn insert() {
        let mut mutable_table: MutableTable<usize> = MutableTable::new();
        mutable_table.insert("example", 13);
        assert!(!mutable_table.is_empty());
        assert_eq!(*mutable_table.get("example").unwrap(), 13);
    }

    #[test]
    fn insert_is_mutable() {
        let mut mutable_table:MutableTable<usize> = MutableTable::new();
        mutable_table.insert("example", 13);
        assert_eq!(*mutable_table.get("example").unwrap(), 13);
        mutable_table.insert("example", 14);
        assert_eq!(*mutable_table.get("example").unwrap(), 14);
    }
}
