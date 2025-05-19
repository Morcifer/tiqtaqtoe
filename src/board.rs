use std::collections::{HashMap, HashSet};
use std::fmt;

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
pub struct SpookyMark(pub Position, pub Position, pub TurnToken);

const BOARD_SIZE: usize = 3;

pub struct Board {
    pub positions: Vec<Position>,
    pub board: [[Option<TurnToken>; BOARD_SIZE]; BOARD_SIZE], // the board is only updated on collapses
    pub turn: u8,
    pub spooky_marks: Vec<SpookyMark>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            positions: (0..BOARD_SIZE)
                .flat_map(|row| (0..BOARD_SIZE).map(move |column| Position::new(row, column)))
                .collect_vec(),
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            turn: 1,
            spooky_marks: vec![],
        }
    }

    pub fn set_mark(&mut self, position: Position, turn_token: TurnToken) {
        self.board[position.row][position.column] = Some(turn_token);
    }

    pub fn get_mark(&self, position: Position) -> Option<TurnToken> {
        self.board[position.row][position.column]
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
            self.set_mark(position_1, turn_token);
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
        // Find if there is a loop
        let mut visited: HashMap<Position, bool> =
            self.positions.iter().map(|p| (*p, false)).collect();

        let mut parents: HashMap<Position, Option<Position>> =
            self.positions.iter().map(|p| (*p, None)).collect();

        let (mut start_loop, mut end_loop) = (None, None);

        for position in &self.positions {
            if !visited[position] {
                if let Some((start, end)) = self.depth_first_search(
                    *position,
                    parents[position],
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

        self.set_mark(position, first.2);

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

            if self.get_mark(to_collapse.0).is_some() {
                println!("collapsing {to_collapse:?} on {}", to_collapse.1);
                self.set_mark(to_collapse.1, to_collapse.2);
                collapsed_positions.insert(to_collapse.1);
            } else if self.get_mark(to_collapse.1).is_some() {
                println!("collapsing {to_collapse:?} on {}", to_collapse.0);
                self.set_mark(to_collapse.0, to_collapse.2);
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
                    .map(|position| match self.get_mark(*position) {
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
                    .map(|position| match self.get_mark(*position) {
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

const X0: &[char] = &[' ', 'X', 'X', ' ', ' ', 'X', 'X', ' '];
const X1: &[char] = &[' ', ' ', ' ', 'X', 'X', ' ', ' ', ' '];

const O0: &[char] = &[' ', ' ', 'O', 'O', 'O', 'O', ' ', ' '];
const O1: &[char] = &[' ', 'O', 'O', ' ', ' ', 'O', 'O', ' '];

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = [[' '; 32]; 11]; // TODO: Make generic for grid size!

        for row in &mut display {
            row[10] = '│';
            row[21] = '│';
        }

        // TODO: Make const of this.
        display[3] = "──────────┼──────────┼──────────"
            .chars()
            .collect_vec()
            .try_into()
            .unwrap();

        display[7] = "──────────┼──────────┼──────────"
            .chars()
            .collect_vec()
            .try_into()
            .unwrap();

        for position in &self.positions {
            let row_offset = position.row * 4;
            let column_offset = position.column * 11;

            match self.get_mark(*position) {
                Some(TurnToken::X(turn)) => {
                    display[row_offset][column_offset..column_offset + 8].copy_from_slice(X0);
                    display[row_offset + 1][column_offset..column_offset + 8].copy_from_slice(X1);
                    display[row_offset + 2][column_offset..column_offset + 8].copy_from_slice(X0);

                    display[row_offset + 2][column_offset + 8] =
                        char::from_digit(turn as u32, 10).unwrap();
                }
                Some(TurnToken::O(turn)) => {
                    display[row_offset][column_offset..column_offset + 8].copy_from_slice(O0);
                    display[row_offset + 1][column_offset..column_offset + 8].copy_from_slice(O1);
                    display[row_offset + 2][column_offset..column_offset + 8].copy_from_slice(O0);

                    display[row_offset + 2][column_offset + 8] =
                        char::from_digit(turn as u32, 10).unwrap();
                }
                None => continue,
            }
        }

        for SpookyMark(p1, p2, m) in &self.spooky_marks {
            let row_offset_1 = p1.row * 4;
            let column_offset_1 = p1.column * 11;

            let row_offset_2 = p2.row * 4;
            let column_offset_2 = p2.column * 11;

            let slice = match m {
                TurnToken::X(turn) => ['X', char::from_digit(*turn as u32, 10).unwrap()],
                TurnToken::O(turn) => ['O', char::from_digit(*turn as u32, 10).unwrap()],
            };

            display[row_offset_1 + p2.row]
                [column_offset_1 + p2.column * 3 + 1..column_offset_1 + p2.column * 3 + 3]
                .copy_from_slice(&slice);
            display[row_offset_2 + p1.row]
                [column_offset_2 + p1.column * 3 + 1..column_offset_2 + p1.column * 3 + 3]
                .copy_from_slice(&slice);
        }

        for row in display {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }

        Ok(())
    }
}
