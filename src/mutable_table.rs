use std::collections::HashMap;
use table::Table;

pub struct MutableTable<T>(HashMap<String, T>);

impl<T> MutableTable<T> {
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
