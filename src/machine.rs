use std::fmt;
use stack::Stack;
use frame::Frame;
use table::Table;
use builder::Builder;

pub struct Machine<'a, T: 'a + fmt::Display> {
    pub builder: Builder<'a, T>,
    pub ip: usize,
    pub constants: &'a Table<Item = T>,
    pub call_stack: Stack<Frame<T>>,
    pub operand_stack: Stack<T>,
}

impl<'a, T: 'a + fmt::Display> Machine<'a, T> {
    pub fn new(builder: Builder<'a, T>, constants: &'a Table<Item = T>) -> Machine<'a, T> {
        let frame: Frame<T> = Frame::new(0);
        let mut call_stack = Stack::new();
        call_stack.push(frame);

        Machine{
            builder:       builder,
            ip:            0,
            constants:     constants,
            call_stack:    call_stack,
            operand_stack: Stack::new()
        }
    }

    pub fn run(mut machine: Machine<'a, T>) -> Machine<'a, T> {
        loop {
            if machine.ip == machine.builder.len() { break; }

            let op_code = machine.builder.instructions[machine.ip];
            let instr = machine
                .builder
                .instruction_table
                .by_op_code(op_code)
                .expect(&format!("Unable to find instruction with op code {}", op_code));

            machine.ip = machine.ip + 1;
            let mut args: Vec<usize> = vec![];

            for _i in 0..instr.arity {
                args.push(machine.builder.instructions[machine.ip]);
                machine.ip = machine.ip + 1;
            }

            let fun = instr.fun;
            fun(&mut machine, args.as_slice());
        }
        machine
    }

    pub fn get_local(&self, name: &str) -> Option<&T> {
        self.call_stack
            .peek()
            .get_local(name)
    }

    pub fn set_local(&mut self, name: &str, value: T) {
        self.call_stack
            .peek_mut()
            .set_local(name, value)
    }

    pub fn operand_push(&mut self, value: T) {
        self.operand_stack
            .push(value);
    }

    pub fn operand_pop(&mut self) -> T {
        self.operand_stack
            .pop()
    }

    pub fn get_data(&self, idx: usize) -> &T {
        self.builder
            .data
            .get(idx)
            .expect(&format!("Constant data is not present at index {}.", idx))
    }

    pub fn jump(&mut self, label: &str) {
        let new_ip = self.builder
            .labels
            .get(label)
            .expect(&format!("Attempt to jump to unknown label {}", label));
        let old_ip = self.ip;
        self.call_stack.push(Frame::new(old_ip));
        self.ip = *new_ip;
    }

    pub fn ret(&mut self) {
        let frame = self.call_stack.pop();
        self.ip = frame.return_address;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mutable_table::MutableTable;
    use instruction::Instruction;
    use instruction_table::InstructionTable;

    fn push(machine: &mut Machine<usize>, args: &[usize]) {
        let arg = machine.builder.data.get(args[0]).unwrap();
        machine.operand_stack.push(*arg);
    }

    fn add(machine: &mut Machine<usize>, _args: &[usize]) {
        let rhs = machine.operand_pop();
        let lhs = machine.operand_pop();
        machine.operand_stack.push(lhs + rhs);
    }

    fn instruction_table() -> InstructionTable<usize> {
        let mut it = InstructionTable::new();
        it.insert(Instruction::new(1, "push", 1, push));
        it.insert(Instruction::new(2, "add",  0, add));
        it
    }

    #[test]
    fn new() {
        let it = instruction_table();
        let builder: Builder<usize> = Builder::new(&it);
        let constants: MutableTable<usize> = MutableTable::new();
        let machine = Machine::new(builder, &constants);
        assert_eq!(machine.ip, 0);
        assert!(!machine.call_stack.is_empty());
        assert!(machine.operand_stack.is_empty());
    }

    #[test]
    fn run() {
        let it = instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push(1, vec![2]);
        builder.push(1, vec![3]);
        builder.push(2, vec![]);
        let constants: MutableTable<usize> = MutableTable::new();
        let machine = Machine::new(builder, &constants);
        let mut machine = Machine::run(machine);
        let result = machine.operand_stack.pop();
        assert_eq!(result, 5);
    }

    #[test]
    fn get_local() {
        let it = instruction_table();
        let builder: Builder<usize> = Builder::new(&it);
        let constants: MutableTable<usize> = MutableTable::new();
        let machine = Machine::new(builder, &constants);
        assert!(machine.get_local("example").is_none());
    }

    #[test]
    fn set_local() {
        let it = instruction_table();
        let builder: Builder<usize> = Builder::new(&it);
        let constants: MutableTable<usize> = MutableTable::new();
        let mut machine = Machine::new(builder, &constants);
        assert!(machine.get_local("example").is_none());
        machine.set_local("example", 13);
        assert_eq!(*machine.get_local("example").unwrap(), 13);
    }
}
