use std::collections::HashMap;
use table::Table;

pub struct ImmutableTable<T>(HashMap<String, T>);

impl<T> ImmutableTable<T> {
    pub fn new() -> ImmutableTable<T> {
        ImmutableTable(HashMap::new())
    }

    fn already_exists_guard(&self, name: &str) {
        if self.0.contains_key(name) {
            panic!("Error: redefining constant {} not allowed.", name);
        }
    }
}

impl<T> Table for ImmutableTable<T> {
    type Item = T;

    fn insert(&mut self, name: &str, value: T) {
        self.already_exists_guard(name);
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
        let immutable_table: ImmutableTable<usize> = ImmutableTable::new();
        assert!(immutable_table.is_empty())
    }

    #[test]
    fn insert() {
        let mut immutable_table: ImmutableTable<usize> = ImmutableTable::new();
        immutable_table.insert("example", 13);
        assert!(!immutable_table.is_empty());
        assert_eq!(*immutable_table.get("example").unwrap(), 13);
    }

    #[test]
    #[should_panic(expected = "redefining constant")]
    fn insert_uniq() {
        let mut immutable_table: ImmutableTable<usize> = ImmutableTable::new();
        immutable_table.insert("example", 13);
        assert_eq!(*immutable_table.get("example").unwrap(), 13);
        immutable_table.insert("example", 13);
    }
}
