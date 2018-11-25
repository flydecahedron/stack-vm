//! Test of non-operand labels.
//!
//! Verifies that it's possible to write programs without having to store labels
//! in your operands.
//!
//! ```
//!
//! ```

use *;

/// Pushes a piece of data from the data section onto the operand stack.
fn push(machine: &mut Machine<usize>, args: &[usize]) {
    let arg = *machine.get_data(args[0]);
    machine.operand_push(arg);
}

/// Build an instruction table based on the instructions outlined above.
fn instruction_table() -> InstructionTable<usize> {
    let mut it = InstructionTable::new();
    it.insert(Instruction::new(0, "push", 1, push));
    it
}

fn program() {
    let it = instruction_table();
    let mut builder: Builder<usize> = Builder::new();
}
