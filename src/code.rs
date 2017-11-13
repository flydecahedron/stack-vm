use std::fmt;
use std::io::Write;
use rmp;
use builder::Builder;
use table::Table;
use code_serialize::CodeSerialize;

pub struct Code<T: fmt::Debug> {
    pub symbols: Vec<(usize, String)>,
    pub code:    Vec<usize>,
    pub data:    Vec<T>,
    pub labels:  Vec<(usize, String)>
}

impl<T: fmt::Debug> Code<T> {
    pub fn from_builder(builder: Builder<T>) -> Code<T> {
        let symbols = builder.instruction_table.symbols();
        let code    = builder.instructions;
        let data    = builder.data;
        let mut labels = vec![];
        for key in builder.labels.keys() {
            let idx = builder.labels.get(key).unwrap();
            labels.push((*idx, key.clone()));
        }

        Code {
            symbols: symbols,
            code:    code,
            data:    data,
            labels:  labels
        }
    }

    pub fn symbols(&self) -> &[(usize, String)] {
        self.symbols.as_slice()
    }

    pub fn code(&self) -> &[usize] {
        self.code.as_slice()
    }

    pub fn data(&self) -> &[T] {
        self.data.as_slice()
    }

    pub fn labels(&self) -> &[(usize, String)] {
        self.labels.as_slice()
    }

    pub fn get_label_ip(&self, name: &str) -> Option<usize> {
        for label in self.labels.as_slice() {
            if label.1 == name { return Some(label.0); }
        }
        None
    }
}

impl<T: fmt::Debug> fmt::Debug for Code<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        // Write out constant data into the header.
        for i in 0..self.data.len() {
            result.push_str(&format!("@{} = {:?}\n", i, self.data[i]));
        }

        // Loop through the code and print out useful stuff.
        let mut ip = 0;
        let len = self.code.len();
        loop {
            if ip == len { break; }

            // If this IP has a label, then print it out.
            for label in self.labels() {
                if ip == label.0 {
                    result.push_str(&format!("\n.{}:\n", label.1));
                    break;
                }
            }

            let op_code = self.code[ip];
            let arity   = self.code[ip + 1];
            ip = ip + 2;

            // Print this instruction's name
            for symbol in self.symbols() {
                if op_code == symbol.0 {
                    result.push_str(&format!("\t{}", symbol.1));
                    break;
                }
            }

            for _i in 0..arity {
                let const_idx = self.code[ip];
                ip = ip + 1;
                result.push_str(&format!(" @{}", const_idx));
            }
            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
}

impl<T: CodeSerialize + fmt::Debug> CodeSerialize for Code<T> {
    /// Create bytecode for this `Code`.
    ///
    /// Encodes into a Map of the following format:
    /// ```json
    /// {
    ///     "code" => [ 0, 1, 0, 0, 1, 1, 1, 0 ],
    ///     "data" => [ 123, 456 ],
    ///     "symbols" => [ 0, "push", 1, "add" ],
    ///     "labels" => [ 0, "main" ]
    /// }
    /// ```
    fn to_byte_code(&self, mut buf: &mut Write) {
        // We're creating a 4-element map.
        rmp::encode::write_map_len(&mut buf, 4).unwrap();

        // First, the code.
        rmp::encode::write_str(&mut buf, "code").unwrap();
        rmp::encode::write_array_len(&mut buf, self.code.len() as u32).unwrap();
        for operation in self.code() {
            rmp::encode::write_uint(&mut buf, *operation as u64).unwrap();
        }

        // Next, the data.
        rmp::encode::write_str(&mut buf, "data").unwrap();
        rmp::encode::write_array_len(&mut buf, self.data.len() as u32).unwrap();
        for operand in self.data() {
            operand.to_byte_code(&mut buf);
        }

        // Next, the symbols.
        rmp::encode::write_str(&mut buf, "symbols").unwrap();
        rmp::encode::write_array_len(&mut buf, (self.symbols.len() * 2) as u32).unwrap();
        for symbol in self.symbols() {
            rmp::encode::write_uint(&mut buf, symbol.0 as u64).unwrap();
            rmp::encode::write_str(&mut buf, &symbol.1).unwrap();
        }

        // Lastly, the labels.
        rmp::encode::write_str(&mut buf, "labels").unwrap();
        rmp::encode::write_array_len(&mut buf, (self.labels.len() * 2) as u32).unwrap();
        for label in self.labels() {
            rmp::encode::write_uint(&mut buf, label.0 as u64).unwrap();
            rmp::encode::write_str(&mut buf, &label.1).unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use instruction::Instruction;
    use instruction_table::InstructionTable;
    use machine::Machine;
    use code_serialize::CodeSerialize;

    impl CodeSerialize for usize {
        fn to_byte_code(&self, mut buf: &mut Write) {
            rmp::encode::write_uint(&mut buf, *self as u64).unwrap();
        }
    }

    fn noop(_machine: &mut Machine<usize>, _args: &[usize]) {}

    fn example_instruction_table() -> InstructionTable<usize> {
        let mut it = InstructionTable::new();
        it.insert(Instruction::new(0, "noop", 0, noop));
        it.insert(Instruction::new(1, "push", 1, noop));
        it.insert(Instruction::new(2, "pop", 0, noop));
        it
    }

    #[test]
    fn from_builder() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push(1, vec![13]);
        builder.push(1, vec![14]);
        let code: Code<usize> = Code::from_builder(builder);

        assert_eq!(code.symbols().len(), 3);
        assert_eq!(code.symbols()[0], (0 as usize, "noop".to_string()));
        assert_eq!(code.symbols()[1], (1 as usize, "push".to_string()));
        assert_eq!(code.symbols()[2], (2 as usize, "pop".to_string()));

        assert_eq!(code.code(), [1, 1, 0, 1, 1, 1]);
        assert_eq!(code.data(), [13, 14]);
        assert_eq!(code.labels().len(), 1);
        assert_eq!(code.labels()[0], (0 as usize, "main".to_string()));
    }

    #[test]
    fn get_label_ip() {
        let it = example_instruction_table();
        let builder: Builder<usize> = Builder::new(&it);
        let code: Code<usize> = Code::from_builder(builder);
        assert_eq!(code.get_label_ip("main").unwrap(), 0);
    }

    #[test]
    fn debug_formatter() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push(0, vec![]);
        builder.push(1, vec![123]);
        builder.push(1, vec![456]);
        builder.label("some_function");
        builder.push(2, vec![]);
        let code = Code::from_builder(builder);

        let actual = format!("{:?}", code);
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

    #[test]
    fn to_byte_code() {
        let it = example_instruction_table();
        let mut builder: Builder<usize> = Builder::new(&it);
        builder.push(0, vec![]);
        builder.push(1, vec![123]);
        builder.push(1, vec![456]);
        builder.label("some_function");
        builder.push(2, vec![]);
        let code = Code::from_builder(builder);
        let mut actual: Vec<u8> = vec![];
        code.to_byte_code(&mut actual);
        let expected = [132, 164, 99, 111, 100, 101, 154, 0, 0, 1, 1, 0, 1, 1,
                        1, 2, 0, 164, 100, 97, 116, 97, 146, 123, 205, 1, 200,
                        167, 115, 121, 109, 98, 111, 108, 115, 150, 0, 164,
                        110, 111, 111, 112, 1, 164, 112, 117, 115, 104, 2, 163,
                        112, 111, 112, 166, 108, 97, 98, 101, 108, 115, 148, 8,
                        173, 115, 111, 109, 101, 95, 102, 117, 110, 99, 116,
                        105, 111, 110, 0, 164, 109, 97, 105, 110];
        assert_eq!(&actual[..], &expected[..]);
    }
}
