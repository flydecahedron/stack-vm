use object::Object;

pub trait Table {
    fn insert(&mut self, name: &str, value: Box<Object>);
    fn is_empty(&self) -> bool;
    fn contains_key(&self, name: &str) -> bool;
    fn get(&self, name: &str) -> Option<&Box<Object>>;
}
