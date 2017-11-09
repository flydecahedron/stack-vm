lazy_static! {
    static ref INSTRUCTIONS: Vec<Instruction> = {
        let mut ops = vec![];
        build_op(&mut ops, 0, "noop", 0);
        build_op(&mut ops, 1, "push", 1);
        build_op(&mut ops, 2, "pop",  0);

        ops
    };
}

fn build_op(ops: &mut Vec<Instruction>, op_code: usize, name: &str, arity: usize) {
    ops.push(Instruction { op_code: op_code, name: String::from(name), arity: arity });
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub op_code: usize,
    pub name:    String,
    pub arity:   usize
}

impl Instruction {
    pub fn for_op_code(op_code: usize) -> Instruction {
        INSTRUCTIONS
            .iter()
            .find(|ref i| i.op_code == op_code)
            .expect(&format!("Unknown instruction op code {:?}", op_code))
            .clone()
    }

    fn by_name(name: &str) -> Instruction {
        INSTRUCTIONS
            .iter()
            .find(|ref i| i.name == name)
            .expect(&format!("Unknown instruction {:?}", name))
            .clone()
    }

    pub fn noop() -> Instruction { Instruction::by_name("noop") }
    pub fn push() -> Instruction { Instruction::by_name("push") }
    pub fn pop()  -> Instruction { Instruction::by_name("pop") }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_op_code() {
        let instr = Instruction::for_op_code(0);
        assert_eq!(instr.op_code, 0);
        assert_eq!(instr.name, "noop");
    }

    #[test]
    fn by_name() {
        let instr = Instruction::by_name("noop");
        assert_eq!(instr.op_code, 0);
        assert_eq!(instr.name, "noop");
    }
}
