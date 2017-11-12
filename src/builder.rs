//! The instruction builder.
//!
//! Use this module to build code for your machine to execute.
//!
//! ## Examples
//!
//! ```
//! use stack_vm::{Instruction, InstructionTable, Builder, Machine};
//!
//! fn push(machine: &mut Machine<f64>, args: &[usize]) {
//!     let arg = machine.get_data(args[0]).clone();
//!     machine.operand_push(arg)
//! }
//!
//! let mut instruction_table: InstructionTable<f64> = InstructionTable::new();
//! instruction_table.insert(Instruction::new(0, "push", 1, push));
//!
//! let mut builder: Builder<f64> = Builder::new(&instruction_table);
//! builder.push(0, vec![1.23]);
//! ```

use std::fmt;
use instruction_table::InstructionTable;
use immutable_table::ImmutableTable;
use table::Table;

/// The builder struct.
///
/// Contains:
/// * an `InstructionTable`.
/// * a list of instructions that have been pushed into this builder.
/// * a `Table` of labels used for jumping.
/// * a list of `T` to be stored in the builder's data section.
pub struct Builder<'a, T: 'a + fmt::Debug> {
    pub instruction_table: &'a InstructionTable<T>,
    pub instructions:      Vec<usize>,
    pub labels:            ImmutableTable<usize>,
    pub data:              Vec<T>,
}

impl<'a, T: fmt::Debug>  Builder<'a, T> {
    /// Create a new `Builder` from an `InstructionTable`.
    pub fn new(instruction_table: &'a InstructionTable<T>) -> Builder<T> {
        let mut labels = ImmutableTable::new();
        labels.insert("main", 0);
        Builder {
            instruction_table: &instruction_table,
            instructions:      vec![],
            labels:            labels,
            data:              vec![],
        }
    }

    /// Push an op-code into the code.
    ///
    /// * `op_code` should match that of an instruction in the
    ///   `InstructionTable`.
    /// * `args` a vector of operands to be pushed into the builder's data
    ///   section.
    pub fn push(&mut self, op_code: usize, args: Vec<T>) {
        let instr = self
            .instruction_table
            .by_op_code(op_code)
            .expect(&format!("Unable to find instruction with op code {}", op_code));

        if args.len() != instr.arity {
            panic!("Instruction {} has arity of {}, but you provided {} arguments.", instr.name, instr.arity, args.len())
        }

        self.instructions.push(instr.op_code);
        for arg in args {
            self.data.push(arg);
            self.instructions.push(self.data.len() - 1);
        }
    }

    /// Insert a label at this point in the code.
    ///
    /// Labels are used as targets for jumps.  When you call this method a
    /// label is stored which points to the position of the next instruction.
    pub fn label(&mut self, name: &str) {
        let idx = self.instructions.len();
        self.labels.insert(name, idx);
    }

    /// Return the length of the instructions vector.
    ///
    /// i.e. the number of instructions pushed so far.
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}

impl<'a, T: 'a + fmt::Debug> fmt::Debug for Builder<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for i in 0..self.data.len() {
            result.push_str(&format!("@{} = {:?}\n", i, self.data[i]));
        }

        let mut ip = 0;
        let len = self.instructions.len();
        loop {
            if ip == len { break; }

            for label in self.labels.keys() {
                let idx = *self.labels.get(label).unwrap();
                if idx == ip {
                    result.push_str(&format!("\n.{}:\n", label));
                }
            }

            let op_code = self.instructions[ip];

            let instr = self
                .instruction_table
                .by_op_code(op_code)
                .expect(&format!("Unable to find instruction with op code {}", op_code));

            result.push_str(&format!("\t{}", &instr.name));

            for _j in 0..instr.arity {
                ip = ip + 1;
                let const_idx = self.instructions[ip];
                result.push_str(&format!(" @{}", const_idx));
            }
            result.push_str("\n");

            ip = ip + 1;
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use instruction::Instruction;
    use instruction_table::InstructionTable;
    use machine::Machine;

    fn noop(_machine: &mut Machine<usize>, _args: &[usize]) {}

    fn example_instruction_table() -> InstructionTable<usize> {
        let mut it = InstructionTable::new();
        it.insert(Instruction::new(0, "noop", 0, noop));
        it.insert(Instruction::new(1, "push", 1, noop));
        it.insert(Instruction::new(2, "pop", 0, noop));
        it
    }

    #[test]
    fn new() {
        let it = example_instruction_table();
        let builder: Builder<usize> = Builder::new(&it);
        assert!(builder.instructions.is_empty());
    }

    #[test]
    fn push() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push(0, vec![]);
        assert!(!builder.instructions.is_empty());
    }

    #[test]
    #[should_panic(expected = "has arity of")]
    fn push_with_incorrect_arity() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push(0, vec![1]);
    }

    #[test]
    fn label() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push(0, vec![]);
        builder.label("wow");
        assert_eq!(*builder.labels.get("wow").unwrap(), 1);
    }

    #[test]
    fn debug_format() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push(0, vec![]);
        builder.push(1, vec![123]);
        builder.push(1, vec![456]);
        builder.label("some_function");
        builder.push(2, vec![]);

        let actual = format!("{:?}", builder);
        let expected = "@0 = 123
@1 = 456

.main:
\tnoop
\tpush @0
\tpush @1

.some_function:
\tpop
";
        assert_eq!(actual, expected);
    }
}
