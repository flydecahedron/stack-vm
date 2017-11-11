use table::Table;
use mutable_table::MutableTable;

pub struct Frame<'a, T: 'a> {
    constants: &'a Table<Item = T>,
    locals: MutableTable<T>
}

impl<'a, T: 'a> Frame<'a, T> {
    pub fn new(constants: &'a Table<Item = T>) -> Frame<'a, T> {
        Frame {
            constants: constants,
            locals: MutableTable::new()
        }
    }
}

#[cfg(test)]
mod test {
    use table::Table;
    use immutable_table::ImmutableTable;
    use super::*;

    #[test]
    fn new_has_constants() {
        let constants: ImmutableTable<usize> = ImmutableTable::new();
        let frame: Frame<usize> = Frame::new(&constants);
        assert!(frame.constants.is_empty())
    }

    #[test]
    fn new_has_locals() {
        let constants: ImmutableTable<usize> = ImmutableTable::new();
        let frame: Frame<usize> = Frame::new(&constants);
        assert!(frame.locals.is_empty())
    }
}
