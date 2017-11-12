//! The machine that makes the magic happen.
//!
//! Pour all your ingredients into `Machine` and make it dance.

use std::fmt;
use stack::Stack;
use frame::Frame;
use table::Table;
use builder::Builder;

/// `Machine` contains all the information needed to run your program.
///
/// * A `Builder`, used describe the source instructions and data to execute.
/// * An instruction pointer, which points to the currently-executing
///   instruciton.
/// * A `Table` of constants, which you can use in your instructions if needed.
/// * A `Stack` of `Frame` used to keep track of calls being executed.
/// * A `Stack` of `T` which is used as the main operand stack.
pub struct Machine<'a, T: 'a + fmt::Debug> {
    pub builder: Builder<'a, T>,
    pub ip: usize,
    pub constants: &'a Table<Item = T>,
    pub call_stack: Stack<Frame<T>>,
    pub operand_stack: Stack<T>,
}

impl<'a, T: 'a + fmt::Debug> Machine<'a, T> {
    /// Returns a new `Machine` ready to execute instructions.
    ///
    /// The machine is initialised by passing in your `Builder` which contains
    /// all the code and data of your program, and a `Table` of constants.
    pub fn new(builder: Builder<'a, T>, constants: &'a Table<Item = T>) -> Machine<'a, T> {
        let frame: Frame<T> = Frame::new(builder.instructions.len());
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

    /// Run the machine.
    ///
    /// Kick off the process of running the program.
    ///
    /// Steps through the instructions in your program executing them
    /// one-by-one.  Each instruction function is executed, much like a
    /// callback.
    ///
    /// Stops when either the last instruction is executed or when the
    /// last frame is removed from the call stack.
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

    /// Look up a local variable in the current call frame.
    ///
    /// Note that the variable may not be set in the current frame but it's up
    /// to your instruction to figure out how to deal with this situation.
    pub fn get_local(&self, name: &str) -> Option<&T> {
        self.call_stack
            .peek()
            .get_local(name)
    }

    /// Look for a local variable in all call frames.
    ///
    /// The machine will look in each frame in the call stack starting at the
    /// top and moving down until it locates the local variable in question
    /// or runs out of stack frames.
    pub fn get_local_deep(&self, name: &str) -> Option<&T> {
        for frame in self.call_stack.as_slice().iter().rev() {
            let local = frame.get_local(name);
            if local.is_some() { return local; }
        }
        None
    }

    /// Set a local variable in the current call frame.
    ///
    /// Places a value in the frame's local variable table.
    pub fn set_local(&mut self, name: &str, value: T) {
        self.call_stack
            .peek_mut()
            .set_local(name, value)
    }

    /// Push an operand onto the operand stack.
    pub fn operand_push(&mut self, value: T) {
        self.operand_stack
            .push(value);
    }

    /// Pop an operand off the operand stack.
    pub fn operand_pop(&mut self) -> T {
        self.operand_stack
            .pop()
    }

    /// Retrieve a reference to a `T` stored in the builder's data section.
    pub fn get_data(&self, idx: usize) -> &T {
        self.builder
            .data
            .get(idx)
            .expect(&format!("Constant data is not present at index {}.", idx))
    }

    /// Perform a jump to a named label.
    ///
    /// This method performs the following actions:
    /// * Retrieve the instruction pointer for a given label from the builder.
    /// * Create a new frame with it's return address set to the current
    ///   instruction pointer.
    /// * Push the new frame onto the call stack.
    ///
    /// This method specifically does not transfer operands to call arguments.
    pub fn jump(&mut self, label: &str) {
        let new_ip = self.builder
            .labels
            .get(label)
            .expect(&format!("Attempt to jump to unknown label {}", label));
        let old_ip = self.ip;
        self.call_stack.push(Frame::new(old_ip));
        self.ip = *new_ip;
    }

    /// Performs a return.
    ///
    /// This method pops the top frame off the call stack and moves the
    /// instruction pointer back to the frame's return address.
    /// It's up to you to push your return value onto the operand stack (if
    /// your language has such return semantics).
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
        let mut machine = Machine::new(builder, &constants);
        assert!(machine.get_local("example").is_none());
        machine.set_local("example", 13);
        assert!(machine.get_local("example").is_some());
    }

    #[test]
    fn get_local_deep() {
        let it = instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.label("next");

        let constants: MutableTable<usize> = MutableTable::new();
        let mut machine = Machine::new(builder, &constants);
        machine.set_local("outer", 13);
        assert_eq!(*machine.get_local_deep("outer").unwrap(), 13);
        machine.jump("next");
        machine.set_local("outer", 14);
        machine.set_local("inner", 15);
        assert_eq!(*machine.get_local_deep("outer").unwrap(), 14);
        assert_eq!(*machine.get_local_deep("inner").unwrap(), 15);
        machine.ret();
        assert_eq!(*machine.get_local_deep("outer").unwrap(), 13);
        assert!(machine.get_local_deep("inner").is_none());
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
