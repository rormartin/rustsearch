pub mod prio_list;
pub mod queue;
pub mod stack;

pub trait OpenList<T> {
    fn new() -> Self;
    fn add(&mut self, element: T);
    fn get(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn clear(&mut self);
}

pub trait PriorityOpenList<T> {
    fn add(&mut self, element: T, weigth: f32);
    fn get(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn clear(&mut self);
}
