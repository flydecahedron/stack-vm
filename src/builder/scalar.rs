use rug;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Scalar {
    Integer(rug::Integer),
    Float(f64),
    String(String),
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
