use std::collections::HashMap;
use std::boxed::Box;
use table::Table;
use object::Object;

pub struct MutableTable(HashMap<String, Box<Object>>);

impl MutableTable {
    pub fn new() ->MutableTable {
        MutableTable(HashMap::new())
    }
}

impl Table for MutableTable {
    fn insert(&mut self, name: &str, value: Box<Object>) {
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
        let mutable_table = MutableTable::new();
        assert!(mutable_table.is_empty())
    }

    #[test]
    fn insert() {
        let mut mutable_table = MutableTable::new();
        mutable_table.insert("example", Box::new(Canary(13)));
        assert!(!mutable_table.is_empty());
    }

    #[test]
    fn insert_is_mutable() {
        let mut mutable_table = MutableTable::new();
        mutable_table.insert("example", Box::new(Canary(13)));
        mutable_table.insert("example", Box::new(Canary(14)));
    }
}
