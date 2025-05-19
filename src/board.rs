use std::collections::{HashMap, HashSet};

use derive_more::Display;
use itertools::Itertools;
use rand::Rng;

#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
#[display("{{ row: {}, column: {} }}", row, column)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Position {
        Self { row, column }
    }
}

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)] // TODO: Does this actually do anything?
pub enum Token {
    X,
    O,
}

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)] // TODO: Does this actually do anything?
pub enum TurnToken {
    X(u8),
    O(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpookyMark(Position, Position, TurnToken);

const BOARD_SIZE: usize = 3;

pub struct Board {
    board: [[Option<TurnToken>; BOARD_SIZE]; BOARD_SIZE], // the board is only updated on collapses
    turn: u8,
    spooky_marks: Vec<SpookyMark>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            turn: 1,
            spooky_marks: vec![],
        }
    }

    pub fn set_spooky_mark(&mut self, position_1: Position, position_2: Position, token: Token) {
        println!(
            "Putting {token} in {position_1} and {position_2} at turn {}",
            self.turn
        );

        let turn_token = match token {
            Token::X => TurnToken::X(self.turn),
            Token::O => TurnToken::O(self.turn),
        };

        if position_1 == position_2 {
            self.board[position_1.row][position_1.column] = Some(turn_token);
        } else {
            self.spooky_marks
                .push(SpookyMark(position_1, position_2, turn_token));
        }

        self.turn += 1;
    }

    pub fn depth_first_search(
        &self,
        position: Position,
        parent: Option<Position>,
        visited: &mut HashMap<Position, bool>,
        parents: &mut HashMap<Position, Option<Position>>,
    ) -> Option<(Position, Position)> {
        visited.insert(position, true);

        for SpookyMark(position_1, position_2, _) in &self.spooky_marks {
            let optional_u;

            if *position_1 == position {
                optional_u = Some(*position_2);
            } else if *position_2 == position {
                optional_u = Some(*position_1);
            } else {
                continue;
            }

            let v = position;
            let u = optional_u.unwrap();

            if Some(u) == parent {
                continue;
            }

            if visited[&u] {
                // println!("Cycle found, with {u} from {v}");
                return Some((u, v));
            }

            parents.insert(u, Some(v));

            if let Some((cycle_start, cycle_end)) =
                self.depth_first_search(u, parents[&u], visited, parents)
            {
                return Some((cycle_start, cycle_end));
            }
        }

        None
    }

    pub fn collapse_loop(&mut self) {
        // Find if there is a loop - TODO: extract to a separate struct and file
        let positions = (0..BOARD_SIZE)
            .flat_map(|row| (0..BOARD_SIZE).map(move |column| Position::new(row, column)))
            .collect_vec();

        let mut visited: HashMap<Position, bool> = positions.iter().map(|p| (*p, false)).collect();
        let mut parents: HashMap<Position, Option<Position>> =
            positions.iter().map(|p| (*p, None)).collect();

        let (mut start_loop, mut end_loop) = (None, None);

        for position in positions {
            if !visited[&position] {
                if let Some((start, end)) = self.depth_first_search(
                    position,
                    parents[&position],
                    &mut visited,
                    &mut parents,
                ) {
                    println!("I found a loop from {start} to {end}!");
                    let mut current = end;
                    while current != start {
                        // println!("current is {current}");
                        current = parents[&current].unwrap();
                    }
                    (start_loop, end_loop) = (Some(start), Some(end));
                    break;
                }
            }
        }

        if start_loop.is_none() {
            // println!("Is no loop!");
            return;
        }

        let (start_loop, end_loop) = (start_loop.unwrap(), end_loop.unwrap());

        // Resolve loop, if there is one, by randomly choosing option.
        // collapse the first edge
        let first = *self
            .spooky_marks
            .iter()
            .find(|SpookyMark(p1, p2, _)| {
                (*p1 == start_loop && *p2 == end_loop) || (*p2 == start_loop && *p1 == end_loop)
            })
            .unwrap();

        let mut rng = rand::rng();
        let choice = rng.random_bool(0.5);

        let position = if choice { first.0 } else { first.1 };

        println!("Collapsing {first:?} on {position}");

        self.board[position.row][position.column] = Some(first.2);

        self.spooky_marks.retain(|m| *m != first);

        let mut collapsed_positions: HashSet<Position> = HashSet::new();
        collapsed_positions.insert(position);

        while let Some(to_collapse) = self
            .spooky_marks
            .iter()
            .cloned()
            .find(|m| collapsed_positions.contains(&m.0) || collapsed_positions.contains(&m.1))
        {
            self.spooky_marks.retain(|m| *m != to_collapse);

            if self.board[to_collapse.0.row][to_collapse.0.column].is_some() {
                println!("collapsing {to_collapse:?} on {}", to_collapse.1);
                self.board[to_collapse.1.row][to_collapse.1.column] = Some(to_collapse.2);
                collapsed_positions.insert(to_collapse.1);
            } else if self.board[to_collapse.1.row][to_collapse.1.column].is_some() {
                println!("collapsing {to_collapse:?} on {}", to_collapse.0);
                self.board[to_collapse.0.row][to_collapse.0.column] = Some(to_collapse.2);
                collapsed_positions.insert(to_collapse.0);
            } else {
                continue;
            }
        }
    }

    // TODO: Add tests for a bunch of cases
    pub fn find_winner(&self) -> (usize, usize) {
        // Points X, points O
        // TODO: Extract into list made in constructor!
        let mut rows_columns_and_diagonals: Vec<Vec<Position>> = vec![];

        for row_index in 0..BOARD_SIZE {
            rows_columns_and_diagonals.push(
                (0..BOARD_SIZE)
                    .map(|column_index| Position::new(row_index, column_index))
                    .collect(),
            );
        }

        for column_index in 0..BOARD_SIZE {
            rows_columns_and_diagonals.push(
                (0..BOARD_SIZE)
                    .map(|row_index| Position::new(row_index, column_index))
                    .collect(),
            );
        }

        rows_columns_and_diagonals.push(
            (0..BOARD_SIZE)
                .map(|row_index| Position::new(row_index, row_index))
                .collect(),
        );
        rows_columns_and_diagonals.push(
            (0..BOARD_SIZE)
                .map(|row_index| Position::new(row_index, BOARD_SIZE - row_index - 1))
                .collect(),
        );

        // TODO: extract to reduce repeated code.
        let x_wins = rows_columns_and_diagonals
            .iter()
            .filter_map(|v| {
                let max_turn = v
                    .iter()
                    .map(|Position { row, column }| match self.board[*row][*column] {
                        Some(TurnToken::X(turn)) => turn,
                        _ => u8::MAX,
                    })
                    .max()
                    .unwrap();

                if max_turn == u8::MAX {
                    return None;
                }

                Some(max_turn)
            })
            .max();

        let o_wins = rows_columns_and_diagonals
            .iter()
            .filter_map(|v| {
                let max_turn = v
                    .iter()
                    .map(|Position { row, column }| match self.board[*row][*column] {
                        Some(TurnToken::O(turn)) => turn,
                        _ => u8::MAX,
                    })
                    .max()
                    .unwrap();

                if max_turn == u8::MAX {
                    return None;
                }

                Some(max_turn)
            })
            .max();

        match (x_wins, o_wins) {
            (None, None) => (0, 0),
            (Some(_), None) => (2, 0),
            (None, Some(_)) => (0, 2),
            (Some(turn_x), Some(turn_o)) if turn_x < turn_o => (2, 1),
            (Some(turn_x), Some(turn_o)) if turn_x > turn_o => (1, 2),
            (Some(_), Some(_)) => panic!("I have a tie. I shouldn't be able to have a tie."),
        }
    }
}
