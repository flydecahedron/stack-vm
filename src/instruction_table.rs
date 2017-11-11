use instruction::Instruction;
use std::collections::HashMap;

pub struct InstructionTable(HashMap<usize, Instruction>);

impl InstructionTable {
    pub fn new() -> InstructionTable {
        InstructionTable(HashMap::new())
    }

    pub fn by_op_code(&self, op_code: usize) -> Option<&Instruction> {
        self.0.get(&op_code)
    }

    pub fn by_name(&self, name: &str) -> Option<&Instruction> {
        self.0
            .values()
            .find(|ref instr| instr.name == name)
    }

    pub fn insert(&mut self, instr: Instruction) {
        self.0.insert(instr.op_code, instr);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let table = InstructionTable::new();
        assert!(table.is_empty())
    }

    #[test]
    fn insert() {
        let mut table = InstructionTable::new();
        assert!(table.is_empty());
        table.insert(Instruction::new(0, "NOOP", 0));
        assert!(!table.is_empty());
    }
}
