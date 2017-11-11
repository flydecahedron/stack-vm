use super::super::*;
use std::fmt;

#[derive(Clone)]
enum Operand {
    I(i64),
    S(String)
}

impl Operand {
    fn to_i(&self) -> Option<i64> {
        match self {
            &Operand::I(i) => Some(i),
            _ => None
        }
    }

    fn to_s(&self) -> Option<&str> {
        match self {
            &Operand::S(ref s) => Some(s),
            _ => None
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Operand::I(i) => write!(f, "{}", i),
            &Operand::S(ref s) => write!(f, "{:?}", s)
        }
    }
}

fn push(machine: &mut Machine<Operand>, args: &[usize]) {
    let arg = machine.get_data(args[0]).clone();
    machine.operand_push(arg)
}

fn add(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().to_i().unwrap();
    let lhs = machine.operand_pop().to_i().unwrap();
    machine.operand_push(Operand::I(lhs + rhs));
}

fn call(machine: &mut Machine<Operand>, args: &[usize]) {
    let label = machine.get_data(args[0]).to_s().unwrap();
    // let label = machine.builder.data[args[0]].to_s().unwrap();
    machine.jump(label);
}

fn ret(machine: &mut Machine<Operand>, _args: &[usize]) {
    machine.ret();
}

fn instruction_table() -> InstructionTable<Operand> {
    let mut it = InstructionTable::new();
    it.insert(Instruction::new(0, "push", 1, push));
    it.insert(Instruction::new(1, "add",  0, add));
    it.insert(Instruction::new(2, "call", 1, call));
    it.insert(Instruction::new(3, "ret",  0, ret));
    it
}

fn op_i(i: i64) -> Operand {
    Operand::I(i)
}

fn op_s(s: &str) -> Operand {
    Operand::S(String::from(s))
}

#[test]
fn example() {
    let it = instruction_table();
    let mut builder: Builder<Operand> = Builder::new(&it);
    builder.push(0, vec![op_i(3)]);
    builder.push(0, vec![op_i(4)]);
    builder.push(2, vec![op_s("add_fun")]);
    builder.label("add_fun");
    builder.push(1, vec![]);
    builder.push(3, vec![]);
    let constants: MutableTable<Operand> = MutableTable::new();
    let machine: Machine<Operand> = Machine::new(builder, &constants);
    let mut machine = Machine::run(machine);
    let result = machine.operand_pop().to_i().unwrap();
    assert_eq!(result, 7);
}
