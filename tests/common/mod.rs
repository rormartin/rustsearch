use rustsearch::search::{Action, State, StateHeuristic};
use std::cmp::{Ord, Ordering};
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Operation {
    Sum,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct NumberAction {
    n1: i32,
    n2: i32,
    op: Operation,
}

impl Action for NumberAction {
    fn cost(&self) -> f32 {
        1.0
    }
}

impl NumberAction {
    pub fn build(n1: i32, n2: i32, op: Operation) -> Self {
        if n2 < n1 && (op == Operation::Sum || op == Operation::Mul) {
            NumberAction { n2, n1, op }
        } else {
            NumberAction { n1, n2, op }
        }
    }

    fn operation_result(&self) -> Option<i32> {
        match self.op {
            Operation::Sum => Some(self.n1 + self.n2),
            Operation::Sub => Some(self.n1 - self.n2),
            Operation::Mul => Some(self.n1 * self.n2),
            Operation::Div => {
                if self.n2 != 0 && (self.n1 % self.n2 == 0) {
                    Some(self.n1 / self.n2)
                } else {
                    None
                }
            }
        }
    }
}

impl Display for NumberAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let symbol = match self.op {
            Operation::Sum => "+",
            Operation::Sub => "-",
            Operation::Mul => "*",
            Operation::Div => "/",
        };
        write!(f, "[ {} {} {} ]", self.n1, symbol, self.n2)
    }
}

impl PartialOrd for NumberAction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.operation_result() {
            n if n > other.operation_result() => Some(Ordering::Greater),
            n if n < other.operation_result() => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Ord for NumberAction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NumberState {
    pub numbers: Vec<i32>,
    pub goal: i32,
    pub actions: Vec<NumberAction>,
}

impl Clone for NumberState {
    fn clone(&self) -> NumberState {
        NumberState {
            numbers: self.numbers.to_vec(),
            actions: self.actions.to_vec(),
            goal: self.goal,
        }
    }
}

impl NumberState {
    pub fn build(numbers: Vec<i32>, goal: i32) -> Self {
        NumberState {
            numbers,
            goal,
            actions: vec![],
        }
    }
}

impl State<NumberAction> for NumberState {
    fn apply_action(&self, action: &NumberAction) -> Self {
        let mut new_state = self.clone();
        let mut numbers_to_delete = Vec::new();
        let mut n1_deleted = false;
        let mut n2_deleted = false;

        for (i, n) in new_state.numbers.iter().enumerate() {
            if *n == action.n1 && !n1_deleted {
                n1_deleted = true;
                numbers_to_delete.push(i);
                continue;
            }
            if *n == action.n2 && !n2_deleted {
                n2_deleted = true;
                numbers_to_delete.push(i);
                continue;
            }
        }
        numbers_to_delete.sort();
        new_state.numbers.remove(numbers_to_delete[0]);
        new_state.numbers.remove(numbers_to_delete[1] - 1);

        new_state.numbers.push(action.operation_result().unwrap());
        new_state.numbers.sort();
        new_state.actions.push(*action);
        new_state.actions.sort();
        new_state
    }

    fn get_partial_solution(&self) -> Vec<NumberAction> {
        self.actions.to_vec()
    }

    fn get_solution_cost(&self) -> f32 {
        self.actions
            .iter()
            .map(|x| x.cost())
            .reduce(|x, y| x + y)
            .unwrap_or(0 as f32)
    }

    fn get_applicable_actions(&self) -> Vec<NumberAction> {
        let mut actions = vec![];
        for i1 in 0..(self.numbers.len() - 1) {
            let n1 = self.numbers[i1];
            for i2 in (i1 + 1)..self.numbers.len() {
                let n2 = self.numbers[i2];

                let all_actions = vec![
                    NumberAction::build(n1, n2, Operation::Sum),
                    NumberAction::build(n1, n2, Operation::Sub),
                    NumberAction::build(n2, n1, Operation::Sub),
                    NumberAction::build(n1, n2, Operation::Mul),
                    NumberAction::build(n1, n2, Operation::Div),
                    NumberAction::build(n2, n1, Operation::Div),
                ];

                all_actions
                    .iter()
                    .filter(|a| match a.operation_result() {
                        Some(r) => r > 0,
                        None => false,
                    })
                    .copied()
                    .for_each(|a| actions.push(a));
            }
        }
        actions
    }

    fn is_solution(&self) -> bool {
        self.numbers.contains(&self.goal)
    }

    fn get_state_level(&self) -> usize {
        self.actions.len()
    }
}

impl StateHeuristic for NumberState {
    /// Heuristic that identifies as better state (lower value) the
    /// one with a number closed to the solution.
    fn heuristic(&self) -> f32 {
        let mindiff = self
            .numbers
            .iter()
            .map(|n| (self.goal - n).abs())
            .reduce(std::cmp::min)
            .unwrap_or(self.goal);

        (mindiff / self.goal) as f32
    }
}
