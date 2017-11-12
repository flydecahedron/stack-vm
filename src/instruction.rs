use std::fmt;
use machine::Machine;

pub struct Instruction<T: fmt::Display + fmt::Debug> {
    pub op_code: usize,
    pub name:    String,
    pub arity:   usize,
    pub fun:     fn(&mut Machine<T>, &[usize])
}

impl<T: fmt::Display + fmt::Debug> fmt::Debug for Instruction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instruction {{ op_code: {}, name: {}, arity: {} }}", self.op_code, self.name, self.arity)
    }
}

impl<T: fmt::Display + fmt::Debug> Instruction<T> {
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
