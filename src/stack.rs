use std::fmt;

#[derive(Debug)]
pub struct Stack<T>(Vec<T>);

impl<T: fmt::Debug> Stack<T> {
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

    pub fn peek(&self) -> &T {
        let len = self.0.len();
        if len == 0 { panic!("Cannot peek into empty stack!") }
        &self.0[len - 1]
    }

    pub fn peek_mut(&mut self) -> &mut T {
        let len = self.0.len();
        if len == 0 { panic!("Cannot peek into empty stack!") }
        &mut self.0[len - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let stack: Stack<usize> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(13);
        assert!(!stack.is_empty());
    }

    #[test]
    fn pop() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(13);
        let value = stack.pop();
        assert_eq!(value, 13);
    }

    #[test]
    #[should_panic(expected = "empty stack")]
    fn empty_pop() {
        let mut stack: Stack<usize> = Stack::new();
        stack.pop();
    }

    #[test]
    fn peek() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(13);
        assert_eq!(*stack.peek(), 13)
    }

    #[test]
    #[should_panic(expected = "empty stack")]
    fn empty_peek() {
        let stack: Stack<usize> = Stack::new();
        stack.peek();
    }
}
