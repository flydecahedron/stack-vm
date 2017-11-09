use rug;
use std::fmt;

use instructions::Instruction;

#[derive(Debug, PartialEq)]
enum Scalar {
    Integer(rug::Integer),
    Float(f64),
    String(String),
}

pub struct Builder {
    instructions: Vec<usize>,
    constants:    Vec<Scalar>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            instructions: vec![],
            constants:    vec![],
        }
    }

    pub fn noop(&mut self) {
        let noop = Instruction::noop();
        self.instructions.push(noop.op_code);
    }

    pub fn push_int(&mut self, integer: &rug::Integer) {
        let push = Instruction::push();
        self.constants.push(Scalar::Integer(integer.clone()));
        let pos = self.constants.len() - 1;
        self.instructions.push(push.op_code);
        self.instructions.push(pos);
    }

    pub fn push_float(&mut self, float: f64) {
        let push = Instruction::push();
        self.constants.push(Scalar::Float(float));
        let pos = self.constants.len() - 1;
        self.instructions.push(push.op_code);
        self.instructions.push(pos);
    }

    pub fn push_string(&mut self, string: &str) {
        let push = Instruction::push();
        self.constants.push(Scalar::String(String::from(string)));
        let pos = self.constants.len() - 1;
        self.instructions.push(push.op_code);
        self.instructions.push(pos);
    }

    pub fn pop(&mut self) {
        let pop = Instruction::pop();
        self.instructions.push(pop.op_code);
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Scalar::Integer(ref i) => write!(f, "{}", i),
            &Scalar::Float(ref r)   => write!(f, "{}", r),
            &Scalar::String(ref s)  => write!(f, "{:?}", s)
        }
    }
}

impl fmt::Display for Builder {
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

            let instr = Instruction::for_op_code(self.instructions[i]);
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

    #[test]
    fn new() {
        let builder = Builder::new();
        assert!(builder.instructions.is_empty());
    }

    #[test]
    fn push_int() {
        let mut builder = Builder::new();
        let mut big_int = rug::Integer::new();
        big_int
            .assign_str("1606938044258990275541962092341162602522202993782792835301376")
            .unwrap();
        builder.push_int(&big_int);

        assert_eq!(builder.instructions.len(), 2);
        assert_eq!(builder.instructions[0], 1);
        assert_eq!(builder.instructions[1], 0);
        assert_eq!(builder.constants.len(), 1);
        assert_eq!(builder.constants[0], Scalar::Integer(big_int));
    }

    #[test]
    fn push_float() {
        let mut builder = Builder::new();
        builder.push_float(1.23);

        assert_eq!(builder.instructions.len(), 2);
        assert_eq!(builder.instructions[0], 1);
        assert_eq!(builder.instructions[1], 0);
        assert_eq!(builder.constants.len(), 1);
        assert_eq!(builder.constants[0], Scalar::Float(1.23));
    }

    #[test]
    fn push_string() {
        let mut builder = Builder::new();
        builder.push_string("Marty McFly");
        assert_eq!(builder.instructions.len(), 2);
        assert_eq!(builder.instructions[0], 1);
        assert_eq!(builder.instructions[1], 0);
        assert_eq!(builder.constants.len(), 1);
        assert_eq!(builder.constants[0], Scalar::String(String::from("Marty McFly")));
    }

    #[test]
    fn pop() {
        let mut builder = Builder::new();
        builder.pop();
        assert_eq!(builder.instructions.len(), 1);
        assert_eq!(builder.instructions[0], 2);
        assert_eq!(builder.constants.len(), 0);
    }

    #[test]
    fn serialisation() {
        let mut builder = Builder::new();
        let int = rug::Integer::from(123);
        builder.noop();
        builder.push_int(&int);
        builder.push_float(1.23);
        builder.push_string("Marty McFly");
        let serialised = format!("{}", builder);
        let expected = "@0 = 123\n@1 = 1.23\n@2 = \"Marty McFly\"\n\nnoop\npush @0\npush @1\npush @2\n";
        assert_eq!(serialised, expected);
    }
}
