//! Code deserializer.
//!
//! If you wish to be able to load serialized bytecode into your virtual
//! machine to use again.

/// CodeDeserialize
///
/// This trait represents the ability to load your Operands from bytecode.
/// `stack-vm` uses the [`rmp`](https://crates.io/crates/rmp) crate to load
/// bytecode from a MsgPack encoded binary.
///
/// See the [`rmp` docs](https://docs.rs/rmp/0.8.7/rmp/) to find out which
/// functions you can use to write out your types.

pub trait CodeDeserialize {
    fn from_byte_code(&mut &[u8]) -> Self;
}

#[cfg(test)]
mod test {
    use super::*;
    use rmp;

    #[derive(PartialEq, Debug)]
    struct Operand(i64);

    impl CodeDeserialize for Operand {
        fn from_byte_code(buf: &mut &[u8]) -> Operand {
            let i = rmp::decode::read_int(&mut &buf[..]).unwrap();
            Operand(i)
        }
    }

    #[test]
    fn from_byte_code() {
        let bytecode = [0xd];
        assert_eq!(Operand(13), Operand::from_byte_code(&mut &bytecode[..]));
    }
}
