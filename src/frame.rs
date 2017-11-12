use mutable_table::MutableTable;
use table::Table;

#[derive(Debug)]
pub struct Frame<T> {
    locals: MutableTable<T>,
    pub return_address: usize
}

impl<T> Frame<T> {
    pub fn new(return_address: usize) -> Frame<T> {
        Frame {
            locals: MutableTable::new(),
            return_address: return_address
        }
    }

    pub fn get_local(&self, name: &str) -> Option<&T> {
        self.locals.get(name)
    }

    pub fn set_local(&mut self, name: &str, value: T) {
        self.locals.insert(name, value);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_has_locals() {
        let frame: Frame<usize> = Frame::new(0);
        assert!(frame.locals.is_empty())
    }
}
