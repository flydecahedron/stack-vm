pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> T {
        self.0.pop().expect("Unable to pop from empty stack!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Canary(usize);
    impl Canary {
        fn to_i(&self) -> usize {
            self.0
        }
    }

    #[test]
    fn new() {
        let stack: Stack<Canary> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push() {
        let mut stack: Stack<Canary> = Stack::new();
        stack.push(Canary(13));
        assert!(!stack.is_empty());
    }

    #[test]
    fn pop() {
        let mut stack: Stack<Canary> = Stack::new();
        stack.push(Canary(13));
        let value = stack.pop().to_i();
        assert_eq!(value, 13);
    }

    #[test]
    #[should_panic(expected = "empty stack")]
    fn empty_pop() {
        let mut stack: Stack<Canary> = Stack::new();
        stack.pop();
    }
}
