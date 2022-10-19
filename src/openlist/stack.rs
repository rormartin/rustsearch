use std::collections::VecDeque;

use super::OpenList;

pub struct Stack<T> {
    stack: VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            stack: VecDeque::new(),
        }
    }
}

impl<T> OpenList<T> for Stack<T> {
    fn new() -> Self {
        Stack::new()
    }

    fn add(&mut self, element: T) {
        self.stack.push_back(element)
    }

    fn get(&mut self) -> Option<T> {
        self.stack.pop_back()
    }

    fn peek(&self) -> Option<&T> {
        match self.is_empty() {
            true => None,
            false => self.stack.back(),
        }
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn clear(&mut self) {
        self.stack.clear()
    }
}

#[cfg(test)]
mod tests {

    use crate::openlist::stack::Stack;
    use crate::openlist::OpenList;

    #[test]
    fn test_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(0, stack.len());
    }

    #[test]
    fn test_add() {
        let mut stack: Stack<i32> = Stack::new();
        stack.add(1);
        assert_eq!(1, stack.len());
        stack.add(2);
        assert_eq!(2, stack.len());
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_peak_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.peek().is_none());
    }

    #[test]
    fn test_peak() {
        let mut stack: Stack<i32> = Stack::new();
        for i in 0..100 {
            stack.add(i);
        }
        let pre_peek_size = stack.len();
        assert_eq!(Some(&99), stack.peek());
        assert_eq!(pre_peek_size, stack.len());
    }

    #[test]
    fn test_clear_empty() {
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        stack.clear();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut stack: Stack<i32> = Stack::new();
        for i in 0..100 {
            stack.add(i);
        }
        assert!(!stack.is_empty());
        stack.clear();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_clear_add() {
        let mut stack: Stack<i32> = Stack::new();
        for i in 0..100 {
            stack.add(i);
        }
        assert!(!stack.is_empty());
        stack.clear();
        assert!(stack.is_empty());
        for i in 0..100 {
            stack.add(i);
        }
        assert!(!stack.is_empty());
        assert_eq!(100, stack.len());
    }

    #[test]
    fn test_fill_get_until_empty() {
        let mut stack: Stack<i32> = Stack::new();
        let nsample: usize = 100;
        for i in 0..nsample {
            stack.add(i as i32);
        }
        assert_eq!(nsample, stack.len());

        for i in nsample..0 {
            assert_eq!(Some(i as i32), stack.get());
        }
    }
}
