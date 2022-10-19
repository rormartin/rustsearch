mod common;

use rustsearch::search::Search;

use crate::common::{NumberAction, Operation};

#[test]
fn one_step_breadth() {
    let initial_state = common::NumberState::build(vec![2, 4], 6);
    let expected_solutions = vec![NumberAction::build(2, 4, Operation::Sum)];
    let mut search = Search::new();
    let solutions = search.search_breadth_all(initial_state);
    dbg!(&solutions);
    assert_eq!(1, solutions.len());
    assert_eq!(expected_solutions, solutions[0]);
}

#[test]
fn one_step_depth() {
    let initial_state = common::NumberState::build(vec![2, 4], 6);
    let expected_solutions = vec![NumberAction::build(2, 4, Operation::Sum)];
    let mut search = Search::new();
    let solutions = search.search_depth_all(initial_state);
    dbg!(&solutions);
    assert_eq!(1, solutions.len());
    assert_eq!(expected_solutions, solutions[0]);
}

#[test]
fn no_solution_breadth() {
    let initial_state = common::NumberState::build(vec![2, 4], 3);
    let mut search = Search::new();
    let solutions = search.search_breadth_all(initial_state);
    dbg!(&search.statistics);
    assert!(solutions.is_empty());
}

#[test]
fn no_solution_depth() {
    let initial_state = common::NumberState::build(vec![2, 4], 3);
    let mut search = Search::new();
    let solutions = search.search_depth_all(initial_state);
    dbg!(&search.statistics);
    assert!(solutions.is_empty());
}

#[test]
fn no_solution_iter_depth() {
    let initial_state = common::NumberState::build(vec![2, 4], 3);
    let mut search = Search::new();
    let solutions = search.search_iter_depth_first(initial_state, 1);
    dbg!(&search.statistics);
    assert!(solutions.is_none());
}

#[test]
fn no_solution_a_start() {
    let initial_state = common::NumberState::build(vec![2, 4], 3);
    let mut search = Search::new();
    let solutions = search.search_a_start_first(initial_state);
    dbg!(&search.statistics);
    assert!(solutions.is_none());
}

#[test]
fn problem_1_breadth_first() {
    let initial_state = common::NumberState::build(vec![2, 4, 5, 7, 25], 855);
    let mut search = Search::new();
    let solution = search.search_breadth_first(initial_state);
    dbg!(&search.statistics);
    assert!(solution.is_some());
}

#[test]
fn problem_1_breadth_all() {
    let initial_state = common::NumberState::build(vec![2, 4, 5, 7, 25], 855);
    let mut search = Search::new();
    let solutions = search.search_breadth_all(initial_state);
    dbg!(&search.statistics);
    assert_eq!(2, solutions.len());
}

#[test]
fn problem_1_depth_first() {
    let initial_state = common::NumberState::build(vec![2, 4, 5, 7, 25], 855);
    let mut search = Search::new();
    let solution = search.search_depth_first(initial_state);
    dbg!(&search.statistics);
    assert!(solution.is_some());
}

#[test]
fn problem_all_depth_first() {
    let initial_state = common::NumberState::build(vec![2, 4, 5, 7, 25], 855);
    let mut search = Search::new();
    let solutions = search.search_depth_all(initial_state);
    dbg!(&search.statistics);
    assert_eq!(2, solutions.len());
}

#[test]
fn problem_1_iter_depth() {
    let initial_state = common::NumberState::build(vec![2, 4, 5, 7, 25], 855);
    let mut search = Search::new();
    let solution = search.search_iter_depth_first(initial_state, 1);
    dbg!(&search.statistics);
    assert!(solution.is_some());
}

#[test]
fn problem_1_a_star_first() {
    let initial_state = common::NumberState::build(vec![2, 4, 5, 7, 25], 855);
    let mut search = Search::new();
    let solution = search.search_a_start_first(initial_state);
    dbg!(&search.statistics);
    assert!(solution.is_some());
}
