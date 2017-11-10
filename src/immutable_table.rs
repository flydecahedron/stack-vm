use std::collections::HashMap;
use std::boxed::Box;
use object::Object;
use table::Table;

pub struct ImmutableTable(HashMap<String, Box<Object>>);

impl ImmutableTable {
    pub fn new() -> ImmutableTable {
        ImmutableTable(HashMap::new())
    }

    fn already_exists_guard(&self, name: &str) {
        if self.0.contains_key(name) {
            panic!("Error: redefining constant {} not allowed.", name);
        }
    }
}

impl Table for ImmutableTable {
    fn insert(&mut self, name: &str, value: Box<Object>) {
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

    fn get(&self, name: &str) -> Option<&Box<Object>> {
        self.0.get(name)
    }
}

#[cfg(test)]
mod test {
    use table::Table;
    use super::*;

    struct Canary(usize);

    impl Object for Canary {}

    #[test]
    fn new() {
        let immutable_table = ImmutableTable::new();
        assert!(immutable_table.is_empty())
    }

    #[test]
    fn insert() {
        let mut immutable_table = ImmutableTable::new();
        immutable_table.insert("example", Box::new(Canary(13)));
        assert!(!immutable_table.is_empty());
    }

    #[test]
    #[should_panic(expected = "redefining constant")]
    fn insert_uniq() {
        let mut immutable_table = ImmutableTable::new();
        immutable_table.insert("example", Box::new(Canary(13)));
        immutable_table.insert("example", Box::new(Canary(13)));
    }
}
