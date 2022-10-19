use std::cmp;
use std::fmt::{Display, Formatter, Result};
use std::marker;
use std::rc::Rc;

use crate::openlist::prio_list::PrioList;
use crate::openlist::queue::Queue;
use crate::openlist::stack::Stack;
use crate::openlist::{OpenList, PriorityOpenList};

/// Base action defintion applicable to a state
pub trait Action {
    fn cost(&self) -> f32;
}

pub trait State<A: Action> {
    fn apply_action(&self, action: &A) -> Self;
    fn get_partial_solution(&self) -> Vec<A>;
    fn get_solution_cost(&self) -> f32;
    fn get_applicable_actions(&self) -> Vec<A>;
    fn is_solution(&self) -> bool;
    fn get_state_level(&self) -> usize;
}

pub trait StateHeuristic {
    fn heuristic(&self) -> f32;
}

#[derive(Debug)]
pub struct Statistics {
    pub nodes_explored: usize,
    pub max_depth: usize,
    pub solutions: usize,
}

impl Display for Statistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[ nodes_explored: {}, max_depth: {}, solutions: {} ]",
            self.nodes_explored, self.max_depth, self.solutions,
        )
    }
}

#[derive(Debug)]
pub struct Search<T: State<A>, A: Action> {
    pub statistics: Statistics,
    visited: Vec<Rc<T>>,
    _marker: marker::PhantomData<A>,
}

impl<T, A> Default for Search<T, A>
where
    T: State<A> + PartialEq + Clone,
    A: Action,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> Search<T, A>
where
    T: State<A> + PartialEq + Clone,
    A: Action,
{
    pub fn new() -> Self {
        Search {
            statistics: Statistics {
                nodes_explored: 0,
                max_depth: 0,
                solutions: 0,
            },
            visited: Vec::new(),
            _marker: std::marker::PhantomData::default(),
        }
    }

    pub fn search_breadth_all(&mut self, initial_state: T) -> Vec<Vec<A>> {
        self.search_breadth(initial_state, true)
    }

    pub fn search_breadth_first(&mut self, initial_state: T) -> Option<Vec<A>> {
        self.search_breadth(initial_state, false).pop()
    }

    pub fn search_depth_all(&mut self, initial_state: T) -> Vec<Vec<A>> {
        self.search_depth(initial_state, true, 0)
    }

    pub fn search_depth_first(&mut self, initial_state: T) -> Option<Vec<A>> {
        self.search_depth(initial_state, false, 0).pop()
    }

    pub fn search_iter_depth_first(
        &mut self,
        initial_state: T,
        limit_step: usize,
    ) -> Option<Vec<A>> {
        let mut limit = limit_step;
        loop {
            let solution = self.search_depth(initial_state.clone(), false, limit).pop();
            match solution {
                Some(solution) => return Some(solution),
                None => {
                    if limit > self.statistics.max_depth {
                        return None;
                    }
                    limit += limit_step;
                    self.visited.clear();
                }
            }
        }
    }
}

impl<T, A> Search<T, A>
where
    T: State<A> + StateHeuristic + PartialEq,
    A: Action,
{
    pub fn search_a_start_first(&mut self, initial_state: T) -> Option<Vec<A>> {
        let mut open_list: PrioList<Rc<T>> = PrioList::new();
        self.find_solutions_a_start(initial_state, &mut open_list, false, 0)
            .pop()
    }
}

impl<T, A> Search<T, A>
where
    T: State<A> + PartialEq,
    A: Action,
{
    fn search_breadth(&mut self, initial_state: T, all_solutions: bool) -> Vec<Vec<A>> {
        let mut open_list: Queue<Rc<T>> = Queue::new();
        self.find_solutions(initial_state, &mut open_list, all_solutions, 0)
    }

    fn search_depth(&mut self, initial_state: T, all_solutions: bool, limit: usize) -> Vec<Vec<A>> {
        let mut open_list: Stack<Rc<T>> = Stack::new();
        self.find_solutions(initial_state, &mut open_list, all_solutions, limit)
    }

    fn find_solutions(
        &mut self,
        state: T,
        open_list: &mut impl OpenList<Rc<T>>,
        all_solutions: bool,
        limit: usize,
    ) -> Vec<Vec<A>> {
        let mut max_level: usize = 0;
        let mut solutions: Vec<Vec<A>> = Vec::new();

        open_list.clear();
        open_list.add(Rc::new(state));
        while !open_list.is_empty() {
            match open_list.get() {
                None => return solutions,
                Some(current_state) if !self.visited.contains(&current_state) => {
                    self.visited.push(Rc::clone(&current_state));
                    max_level = cmp::max(max_level, current_state.get_state_level());
                    self.statistics.nodes_explored += 1;
                    self.statistics.max_depth = cmp::max(self.statistics.max_depth, max_level);

                    if current_state.is_solution() {
                        self.statistics.solutions += 1;
                        solutions.push(current_state.get_partial_solution());
                        if !all_solutions {
                            return solutions;
                        }
                        continue;
                    }

                    // expand
                    if (limit > 0 && current_state.get_state_level() < limit) || (limit < 1) {
                        current_state
                            .get_applicable_actions()
                            .iter()
                            .map(|a| Rc::new(current_state.apply_action(a)))
                            .filter(|s| !self.visited.contains(s))
                            .for_each(|s| open_list.add(s));
                    }
                }
                // to ignore visited
                _ => continue,
            };
        }
        solutions
    }
}

impl<T, A> Search<T, A>
where
    T: State<A> + StateHeuristic + PartialEq,
    A: Action,
{
    fn find_solutions_a_start(
        &mut self,
        state: T,
        open_list: &mut impl PriorityOpenList<Rc<T>>,
        all_solutions: bool,
        limit: usize,
    ) -> Vec<Vec<A>> {
        let mut max_level: usize = 0;
        let mut solutions: Vec<Vec<A>> = Vec::new();

        open_list.clear();
        let weight = state.get_solution_cost() + state.heuristic();
        open_list.add(Rc::new(state), weight);
        while !open_list.is_empty() {
            match open_list.get() {
                None => return solutions,
                Some(current_state) if !self.visited.contains(&current_state) => {
                    self.visited.push(Rc::clone(&current_state));
                    max_level = cmp::max(max_level, current_state.get_state_level());
                    self.statistics.nodes_explored += 1;
                    self.statistics.max_depth = cmp::max(self.statistics.max_depth, max_level);

                    if current_state.is_solution() {
                        self.statistics.solutions += 1;
                        solutions.push(current_state.get_partial_solution());
                        if !all_solutions {
                            return solutions;
                        }
                        continue;
                    }

                    // expand
                    if (limit > 0 && current_state.get_state_level() < limit) || (limit < 1) {
                        current_state
                            .get_applicable_actions()
                            .iter()
                            .map(|a| Rc::new(current_state.apply_action(a)))
                            .filter(|s| !self.visited.contains(s))
                            .for_each(|s| {
                                let weight = s.get_solution_cost() + s.heuristic();
                                open_list.add(s, weight);
                            });
                    }
                }
                // to ignore visited
                _ => continue,
            };
        }
        solutions
    }
}
