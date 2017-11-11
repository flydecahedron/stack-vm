use table::Table;
use mutable_table::MutableTable;

pub struct Frame<'a, T: 'a> {
    constants: &'a Table<Item = T>,
    locals: MutableTable<T>,
    return_address: usize
}

impl<'a, T: 'a> Frame<'a, T> {
    pub fn new(constants: &'a Table<Item = T>, return_address: usize) -> Frame<'a, T> {
        Frame {
            constants: constants,
            locals: MutableTable::new(),
            return_address: return_address
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
        let frame: Frame<usize> = Frame::new(&constants, 0);
        assert!(frame.constants.is_empty())
    }

    #[test]
    fn new_has_locals() {
        let constants: ImmutableTable<usize> = ImmutableTable::new();
        let frame: Frame<usize> = Frame::new(&constants, 0);
        assert!(frame.locals.is_empty())
    }
}
