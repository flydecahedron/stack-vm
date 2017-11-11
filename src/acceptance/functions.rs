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
    machine.jump(label)
}
