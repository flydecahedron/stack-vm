use instruction::Instruction;
use stack::Stack;
use frame::Frame;
use table::Table;

pub struct Machine<'a, T: 'a> {
    code: Vec<Instruction>,
    ip: usize,
    constants: &'a Table<Item = T>,
    call_stack: Stack<Frame<'a, T>>,
    operand_stack: Stack<T>
}

impl<'a, T: 'a> Machine<'a, T> {
    pub fn new(code: Vec<Instruction>, constants: &'a Table<Item = T>) -> Machine<'a, T> {
        Machine{
            code: code,
            ip: 0,
            constants: constants,
            call_stack: Stack::new(),
            operand_stack: Stack::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mutable_table::MutableTable;

    #[test]
    fn new() {
        let constants: MutableTable<usize> = MutableTable::new();
        let machine = Machine::new(vec![], &constants);
        assert_eq!(machine.ip, 0);
        assert!(machine.call_stack.is_empty());
        assert!(machine.operand_stack.is_empty());
    }
}
