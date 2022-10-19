use std::collections::VecDeque;

use super::PriorityOpenList;

#[derive(Debug)]
pub struct PrioList<T> {
    elements: VecDeque<T>,
    values: VecDeque<f32>,
}

impl<T> PrioList<T> {
    pub fn new() -> Self {
        PrioList {
            elements: VecDeque::new(),
            values: VecDeque::new(),
        }
    }
}

impl<T> PriorityOpenList<T> for PrioList<T> {
    fn add(&mut self, element: T, weigth: f32) {
        let mut pos: usize = match self.is_empty() {
            true => 0,
            false => self.elements.len(),
        };
        for (i, v) in self.values.iter().enumerate() {
            if v >= &weigth {
                pos = i;
                break;
            }
        }
        self.elements.insert(pos, element);
        self.values.insert(pos, weigth);
    }

    fn get(&mut self) -> Option<T> {
        self.values.pop_front();
        self.elements.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        match self.is_empty() {
            true => None,
            false => self.elements.front(),
        }
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn clear(&mut self) {
        self.elements.clear();
        self.values.clear();
    }
}

#[cfg(test)]
mod tests {

    use crate::openlist::prio_list::PrioList;
    use crate::openlist::PriorityOpenList;

    #[test]
    fn test_empty() {
        let list: PrioList<i32> = PrioList::new();
        assert!(list.is_empty());
        assert_eq!(0, list.len());
    }

    #[test]
    fn test_empty_get() {
        let mut list: PrioList<i32> = PrioList::new();
        assert_eq!(None, list.get());
    }

    #[test]
    fn test_empty_peek() {
        let list: PrioList<i32> = PrioList::new();
        assert_eq!(None, list.peek());
    }

    #[test]
    fn test_add() {
        let mut list: PrioList<i32> = PrioList::new();
        list.add(1, 1.0);
        assert!(!list.is_empty());
        assert_eq!(1, list.len());
        list.add(2, 2.0);
        assert!(!list.is_empty());
        assert_eq!(2, list.len());
    }

    #[test]
    fn test_peek() {
        let mut list: PrioList<i32> = PrioList::new();
        let nelem = 100;
        for i in 0..nelem {
            list.add(i, i as f32);
        }
        assert_eq!(nelem as usize, list.len());
        assert_eq!(Some(&0), list.peek());
    }

    #[test]
    fn test_clear() {
        let mut list: PrioList<i32> = PrioList::new();
        list.add(1, 1.0);
        list.add(2, 2.0);
        assert!(!list.is_empty());
        assert_eq!(2, list.len());
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_sort() {
        let mut list: PrioList<i32> = PrioList::new();
        list.add(3, 3.3);
        list.add(2, 2.2);
        list.add(1, 1.1);
        list.add(5, 5.5);
        list.add(4, 4.4);

        for val in vec![1, 2, 3, 4, 5] {
            let res = list.get();
            assert_eq!(Some(val), res);
        }
        assert!(list.is_empty());
    }

    #[test]
    fn test_add_clear_add() {
        let mut list: PrioList<i32> = PrioList::new();
        list.add(1, 1.0);
        assert!(!list.is_empty());
        list.clear();
        assert!(list.is_empty());
        list.add(2, 2.0);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_sequential_get() {
        let mut list: PrioList<i32> = PrioList::new();
        let nelem = 100;
        for i in 0..nelem {
            list.add(i, i as f32);
        }
        assert_eq!(nelem as usize, list.len());
        for i in 0..nelem {
            assert_eq!(Some(i), list.get());
            assert_eq!((nelem - i - 1) as usize, list.len());
        }
        assert!(list.is_empty());
    }
}
