use std::fmt;
use instruction_table::InstructionTable;
use immutable_table::ImmutableTable;
use table::Table;

pub struct Builder<'a, T: fmt::Display + 'a> {
    pub instruction_table: &'a InstructionTable<T>,
    pub instructions:      Vec<usize>,
    pub labels:            ImmutableTable<usize>,
    pub data:              Vec<T>,
}

impl<'a, T: fmt::Display> Builder<'a, T> {
    pub fn new(instruction_table: &'a InstructionTable<T>) -> Builder<T> {
        Builder {
            instruction_table: &instruction_table,
            instructions:      vec![],
            labels:            ImmutableTable::new(),
            data:              vec![],
        }
    }

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

    pub fn label(&mut self, name: &str) {
        let idx = self.instructions.len();
        self.labels.insert(name, idx);
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}

impl<'a, T: fmt::Display> fmt::Display for Builder<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for i in 0..self.data.len() {
            result.push_str(&format!("@{} = {}\n", i, self.data[i]));
        }

        if self.data.len() > 0 {
            result.push_str("\n");
        }

        let mut i = 0;
        let len = self.instructions.len();
        loop {
            if i == len { break; }

            let op_code = self.instructions[i];

            let instr = self
                .instruction_table
                .by_op_code(op_code)
                .expect(&format!("Unable to find instruction with op code {}", op_code));
            result.push_str(&instr.name);

            for _j in 0..instr.arity {
                i = i + 1;
                let const_idx = self.instructions[i];
                result.push_str(&format!(" @{}", const_idx));
            }
            result.push_str("\n");

            i = i + 1;
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
    fn builder_string_format() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push(0, vec![]);
        builder.push(1, vec![123]);
        builder.push(1, vec![456]);
        builder.push(2, vec![]);
        let actual = format!("{}", builder);
        let expected = "@0 = 123
@1 = 456

noop
push @0
push @1
pop
";
        assert_eq!(actual, expected);
    }
}
