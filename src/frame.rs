use table::Table;
use mutable_table::MutableTable;

pub struct Frame<'a> {
    constants: &'a Table,
    locals: MutableTable
}

impl<'a> Frame<'a> {
    pub fn new(constants: &'a Table) -> Frame<'a> {
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
        let constants = ImmutableTable::new();
        let frame = Frame::new(&constants);
        assert!(frame.constants.is_empty())
    }

    #[test]
    fn new_has_locals() {
        let constants = ImmutableTable::new();
        let frame = Frame::new(&constants);
        assert!(frame.locals.is_empty())
    }
}
