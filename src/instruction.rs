//! A virtual instruction.
//!
//! Instructions consist of an op code, a name, an arity and a function.
//!
//! ## Examples
//!
//! A simple `push` instruction, which takes a piece of data from the builder's
//! data space and places it onto the operand stack.
//!
//! ```
//! use stack_vm::{Instruction, Machine};
//!
//! fn push(machine: &mut Machine<u64>, args: &[usize]) {
//!     let arg = machine.get_data(args[0]).clone();
//!     machine.operand_push(arg);
//! }
//!
//! Instruction::new(0, "push", 1, push);
//! ```
//!
//! A `noop` instruction which does nothing.
//!
//! ```
//! use stack_vm::{Instruction, Machine};
//!
//! fn noop(_machine: &mut Machine<u64>, _args: &[usize]) {
//!     println!("noop");
//! }
//! ```
//!
//! A `jump` instruction, which takes the name of a label from the builder's data
//! and then jumps to it.
//!
//! Note that operand types have to implement `std::fmt::Debug` and `std::fmt::Display`.
//!
//! ```
//! use std::fmt;
//! use stack_vm::{Instruction, Machine};
//!
//! #[derive(Debug)]
//! enum Operand { I(i64), S(String) }
//!
//! impl fmt::Display for Operand {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         match self {
//!             &Operand::S(ref s) => write!(f, "{}", s),
//!             &Operand::I(ref i) => write!(f, "{}", i)
//!         }
//!     }
//! }
//!
//! fn jump(machine: &mut Machine<Operand>, args: &[usize]) {
//!     let label = match machine.get_data(args[0]) {
//!         &Operand::S(ref str) => str.clone(),
//!         _ => panic!("Cannot jump to non-string label.")
//!     };
//!     machine.jump(&label);
//! }
//!
//! Instruction::new(1, "jump", 1, jump);
//! ```

use std::fmt;
use machine::Machine;

/// Describes a single instruction which can be used to execute programs.
///
/// Contains:
/// * An op code - a unique integer to identify this instruction.
/// * A name for serialisation and debugging reasons.
/// * An arity - the number of arguments this instruction expects to receive.
/// * A function which is used to execute the instruction.
pub struct Instruction<T: fmt::Display + fmt::Debug> {
    pub op_code: usize,
    pub name:    String,
    pub arity:   usize,
    pub fun:     INSTRUCTION<T>
}

/// The instruction function signature.
///
/// Each instruction is defined in terms of a function which takes a mutable
/// reference to a `Machine` and an array of `usize`.
///
/// Your instruction is able to manipulate the state of the machine as
/// required (by pushing operands to the stack, for example).
///
/// The `args` array contains indexes into the `Builder`'s data section. It's
/// up to your instruction to retrieve said data.
type INSTRUCTION<T> = fn(machine: &mut Machine<T>, args: &[usize]);

impl<T: fmt::Display + fmt::Debug> fmt::Debug for Instruction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instruction {{ op_code: {}, name: {}, arity: {} }}", self.op_code, self.name, self.arity)
    }
}

impl<T: fmt::Display + fmt::Debug> Instruction<T> {
    /// Create a new instruction.
    pub fn new(op_code: usize, name: &str, arity: usize, fun: INSTRUCTION<T>) -> Instruction<T> {
        Instruction {
            op_code: op_code,
            name: String::from(name),
            arity: arity,
            fun: fun
        }

    }
}

#[cfg(test)]
mod test {
}
