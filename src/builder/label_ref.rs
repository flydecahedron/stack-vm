/// A label reference
#[derive(Debug, Clone)]
pub struct LabelRef(usize);

impl LabelRef {
    pub fn new(idx: usize) -> LabelRef {
        LabelRef(idx)
    }

    pub fn idx(&self) -> usize {
        self.0
    }
}
