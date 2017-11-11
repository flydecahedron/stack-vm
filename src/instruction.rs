use std::fmt;
use machine::Machine;

pub struct Instruction<T: fmt::Display> {
    pub op_code: usize,
    pub name:    String,
    pub arity:   usize,
    pub fun:     fn(&mut Machine<T>, &[usize])
}

impl<T: fmt::Display> Instruction<T> {
    pub fn new(op_code: usize, name: &str, arity: usize, fun: fn(&mut Machine<T>, &[usize])) -> Instruction<T> {
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
