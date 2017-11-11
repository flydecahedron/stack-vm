use std::fmt;
mod scalar;
use self::scalar::Scalar;

use instruction_table::InstructionTable;

pub struct Builder<'a> {
    instruction_table: &'a InstructionTable,
    instructions:      Vec<usize>,
    constants:         Vec<Scalar>,
}

impl<'a> Builder<'a> {
    pub fn new(instruction_table: &'a InstructionTable) -> Builder {
        Builder {
            instruction_table: &instruction_table,
            instructions:      vec![],
            constants:         vec![],
        }
    }

    pub fn push(&mut self, op_code: usize, args: Vec<Scalar>) {
        let instr = self
            .instruction_table
            .by_op_code(op_code)
            .expect(&format!("Unable to find instruction with op code {}", op_code));

        if (args.len() != instr.arity) {
            panic!("Instruction {} has arity of {}, but you provided {} arguments.", instr.name, instr.arity, args.len())
        }

        self.instructions.push(instr.op_code);
        for arg in args {
            self.constants.push(arg);
            self.instructions.push(self.constants.len() - 1);
        }
    }
}

impl<'a> fmt::Display for Builder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for i in 0..self.constants.len() {
            result.push_str(&format!("@{} = {}\n", i, self.constants[i]));
        }

        if self.constants.len() > 0 {
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
    use rug;
    use instruction::Instruction;
    use instruction_table::InstructionTable;

    fn example_instruction_table() -> InstructionTable {
        let mut it = InstructionTable::new();
        it.insert(Instruction::new(0, "noop", 0));
        it.insert(Instruction::new(1, "push", 1));
        it.insert(Instruction::new(2, "pop", 0));
        it
    }

    #[test]
    fn new() {
        let it = example_instruction_table();
        let builder = Builder::new(&it);
        assert!(builder.instructions.is_empty());
    }

    #[test]
    fn push() {
        let it = example_instruction_table();
        let mut builder = Builder::new(&it);
        builder.push(0, vec![]);
        assert!(!builder.instructions.is_empty());
    }

    #[test]
    #[should_panic(expected = "has arity of")]
    fn push_with_incorrect_arity() {
        let it = example_instruction_table();
        let mut builder = Builder::new(&it);
        builder.push(0, vec![Scalar::Float(1.0)]);
    }

    #[test]
    fn builder_string_format() {
        let it = example_instruction_table();
        let mut builder = Builder::new(&it);
        builder.push(0, vec![]);
        builder.push(1, vec![Scalar::Integer(rug::Integer::from(123))]);
        builder.push(1, vec![Scalar::Float(3.45)]);
        builder.push(1, vec![Scalar::String(String::from("wat"))]);
        builder.push(2, vec![]);
        let actual = format!("{}", builder);
        let expected = "@0 = 123
@1 = 3.45
@2 = \"wat\"

noop
push @0
push @1
push @2
pop
";
        assert_eq!(actual, expected);
    }
}
