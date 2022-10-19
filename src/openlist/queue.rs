use std::collections::VecDeque;

use super::OpenList;

#[derive(Debug)]
pub struct Queue<T> {
    queue: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            queue: VecDeque::new(),
        }
    }
}

impl<T> OpenList<T> for Queue<T> {
    fn new() -> Self {
        Queue::new()
    }

    fn add(&mut self, element: T) {
        self.queue.push_back(element)
    }

    fn get(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        match self.is_empty() {
            true => None,
            false => self.queue.front(),
        }
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn clear(&mut self) {
        self.queue.clear()
    }
}

#[cfg(test)]
mod tests {

    use crate::openlist::queue::Queue;
    use crate::openlist::OpenList;

    #[test]
    fn test_empty() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(0, queue.len());
    }

    #[test]
    fn test_add() {
        let mut queue: Queue<i32> = Queue::new();
        queue.add(1);
        assert_eq!(1, queue.len());
        queue.add(2);
        assert_eq!(2, queue.len());
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_peak_empty() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.peek().is_none());
    }

    #[test]
    fn test_peak() {
        let mut queue: Queue<i32> = Queue::new();
        for i in 0..100 {
            queue.add(i);
        }
        let pre_peek_size = queue.len();
        assert_eq!(Some(&0), queue.peek());
        assert_eq!(pre_peek_size, queue.len());
    }

    #[test]
    fn test_clear_empty() {
        let mut queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        queue.clear();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut queue: Queue<i32> = Queue::new();
        for i in 0..100 {
            queue.add(i);
        }
        assert!(!queue.is_empty());
        queue.clear();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_clear_add() {
        let mut queue: Queue<i32> = Queue::new();
        for i in 0..100 {
            queue.add(i);
        }
        assert!(!queue.is_empty());
        queue.clear();
        assert!(queue.is_empty());
        for i in 0..100 {
            queue.add(i);
        }
        assert!(!queue.is_empty());
        assert_eq!(100, queue.len());
    }

    #[test]
    fn test_fill_get_until_empty() {
        let mut queue: Queue<i32> = Queue::new();
        let nsample: usize = 100;
        for i in 0..nsample {
            queue.add(i as i32);
        }
        assert_eq!(nsample, queue.len());

        for i in 0..nsample {
            assert_eq!(Some(i as i32), queue.get());
        }
    }
}
