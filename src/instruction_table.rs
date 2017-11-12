use instruction::Instruction;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct InstructionTable<T: fmt::Display + fmt::Debug>(HashMap<usize, Instruction<T>>);

impl<T: fmt::Display + fmt::Debug> InstructionTable<T> {
    pub fn new() -> InstructionTable<T> {
        InstructionTable(HashMap::new())
    }

    pub fn by_op_code(&self, op_code: usize) -> Option<&Instruction<T>> {
        self.0.get(&op_code)
    }

    pub fn by_name(&self, name: &str) -> Option<&Instruction<T>> {
        self.0
            .values()
            .find(|ref instr| instr.name == name)
    }

    pub fn insert(&mut self, instr: Instruction<T>) {
        self.0.insert(instr.op_code, instr);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use machine::Machine;

    fn noop(_machine: &mut Machine<usize>, _args: &[usize]) {}

    #[test]
    fn new() {
        let table: InstructionTable<usize> = InstructionTable::new();
        assert!(table.is_empty())
    }

    #[test]
    fn insert() {
        let mut table: InstructionTable<usize> = InstructionTable::new();
        assert!(table.is_empty());
        table.insert(Instruction::new(0, "NOOP", 0, noop));
        assert!(!table.is_empty());
    }
}
