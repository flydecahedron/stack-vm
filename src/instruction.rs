#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub op_code: usize,
    pub name:    String,
    pub arity:   usize
}

impl Instruction {
    pub fn new(op_code: usize, name: &str, arity: usize) -> Instruction {
        Instruction {
            op_code: op_code,
            name: String::from(name),
            arity: arity
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

}
