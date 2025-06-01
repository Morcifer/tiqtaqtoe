use std::collections::{HashSet, VecDeque};
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

    pub fn is_valid(&self) -> bool {
        self.row < BOARD_SIZE && self.column < BOARD_SIZE
    }
}

// TODO: Consider adding an ordering for position (left to right, top to bottom)

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

impl From<&TurnToken> for Token {
    fn from(token: &TurnToken) -> Self {
        match token {
            TurnToken::X(_) => Token::X,
            TurnToken::O(_) => Token::O,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpookyMark(pub Position, pub Position, pub TurnToken);

const BOARD_SIZE: usize = 3;

pub struct Board {
    pub positions: Vec<Position>,
    pub rows_columns_and_diagonals: Vec<[Position; BOARD_SIZE]>,
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
            rows_columns_and_diagonals: Self::get_rows_columns_and_diagonals(),
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            turn: 1,
            spooky_marks: vec![],
        }
    }

    fn get_rows_columns_and_diagonals() -> Vec<[Position; BOARD_SIZE]> {
        // TODO: This method is only correct for BOARD_SIZE <= 4.
        let mut rows_columns_and_diagonals: Vec<[Position; BOARD_SIZE]> = vec![];

        for index in 0..BOARD_SIZE {
            // Rows
            rows_columns_and_diagonals.push(
                (0..BOARD_SIZE)
                    .map(|column_index| Position::new(index, column_index))
                    .collect_vec()
                    .try_into()
                    .unwrap(),
            );

            // Columns
            rows_columns_and_diagonals.push(
                (0..BOARD_SIZE)
                    .map(|row_index| Position::new(row_index, index))
                    .collect_vec()
                    .try_into()
                    .unwrap(),
            );
        }

        rows_columns_and_diagonals.push(
            (0..BOARD_SIZE)
                .map(|row_index| Position::new(row_index, row_index))
                .collect_vec()
                .try_into()
                .unwrap(),
        );

        rows_columns_and_diagonals.push(
            (0..BOARD_SIZE)
                .map(|row_index| Position::new(row_index, BOARD_SIZE - row_index - 1))
                .collect_vec()
                .try_into()
                .unwrap(),
        );

        rows_columns_and_diagonals
    }

    pub fn get_mark(&self, position: Position) -> Option<TurnToken> {
        self.board[position.row][position.column]
    }

    fn set_mark(&mut self, position: Position, turn_token: TurnToken) {
        println!("Putting classical turn token {turn_token} in {position}");

        if let Some(spot) = self.board[position.row].get_mut(position.column) {
            match spot {
                Some(_) => panic!("Invalid move, position {position} already occupied!"),
                None => *spot = Some(turn_token),
            }
        } else {
            panic!("I seem to be out of bounds at {position}!")
        }
    }

    fn set_spooky_mark(
        &mut self,
        position_1: Position,
        position_2: Position,
        turn_token: TurnToken,
    ) {
        println!("Putting {turn_token:?} in {position_1} and {position_2}");

        self.spooky_marks
            .push(SpookyMark(position_1, position_2, turn_token));
    }

    pub fn do_turn(&mut self, position_1: Position, position_2: Position, token: Token) {
        if !position_1.is_valid() {
            panic!("Invalid position {position_1}!");
        }

        if !position_2.is_valid() {
            panic!("Invalid position {position_2}!");
        }

        let turn_token = match token {
            Token::X => TurnToken::X(self.turn),
            Token::O => TurnToken::O(self.turn),
        };

        if position_1 == position_2 {
            self.set_mark(position_1, turn_token);
        } else {
            self.set_spooky_mark(position_1, position_2, turn_token);
        }

        self.turn += 1;
    }

    fn depth_first_search(&self) -> Option<(Position, Position)> {
        // Size-two loop is simpler by just having a double loop, for now.
        for SpookyMark(a1, b1, t1) in &self.spooky_marks {
            for SpookyMark(a2, b2, t2) in &self.spooky_marks {
                if t1 == t2 {
                    continue;
                }

                if a1 == a2 && b1 == b2 {
                    return Some((*a1, *b1));
                }

                if a1 == b2 && b1 == a2 {
                    return Some((*a1, *b1));
                }
            }
        }

        let mut roots: VecDeque<Position> = self.positions.iter().cloned().collect();
        let mut visited_roots = HashSet::new();

        while let Some(root) = roots.pop_front() {
            if visited_roots.contains(&root) {
                continue;
            }

            let mut queue: VecDeque<(Option<Position>, Position)> = VecDeque::new();
            let mut visited: HashSet<Position> = HashSet::new();

            queue.push_front((None, root));

            while let Some((from, current)) = queue.pop_front() {
                visited.insert(current);
                visited_roots.insert(current);

                for SpookyMark(position_1, position_2, _) in &self.spooky_marks {
                    let target;

                    if *position_1 == current {
                        target = *position_2;
                    } else if *position_2 == current {
                        target = *position_1;
                    } else {
                        continue;
                    }

                    if Some(target) == from {
                        continue;
                    }

                    if from.is_some() && visited.contains(&target) {
                        return Some((current, target));
                    }

                    queue.push_front((Some(current), target));
                }
            }
        }

        None
    }

    pub fn collapse_loop(&mut self) {
        // Find if there is a loop
        let loop_edge = self.depth_first_search();

        if loop_edge.is_none() {
            return;
        }

        let (start_loop, end_loop) = loop_edge.unwrap();

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

            if self.get_mark(to_collapse.0).is_some() {
                self.board[to_collapse.1.row][to_collapse.1.column] = Some(to_collapse.2);
                collapsed_positions.insert(to_collapse.1);
            } else if self.get_mark(to_collapse.1).is_some() {
                self.board[to_collapse.0.row][to_collapse.0.column] = Some(to_collapse.2);
                collapsed_positions.insert(to_collapse.0);
            } else {
                continue;
            }
        }
    }

    fn find_win(&self, token: Token) -> Option<u8> {
        self.rows_columns_and_diagonals
            .iter()
            .filter_map(|v| {
                let max_turn = v
                    .iter()
                    .map(|position| match self.get_mark(*position) {
                        Some(TurnToken::X(turn)) if token == Token::X => turn,
                        Some(TurnToken::O(turn)) if token == Token::O => turn,
                        _ => u8::MAX,
                    })
                    .max()
                    .unwrap();

                if max_turn == u8::MAX {
                    return None;
                }

                Some(max_turn)
            })
            .max()
    }

    // TODO: Add tests for a bunch of cases
    pub fn get_score(&self) -> (f32, f32) {
        let x_wins = self.find_win(Token::X);
        let o_wins = self.find_win(Token::O);

        match (x_wins, o_wins) {
            (None, None) => (0.0, 0.0),
            (Some(_), None) => (1.0, 0.0),
            (None, Some(_)) => (0.0, 1.0),
            (Some(turn_x), Some(turn_o)) if turn_x < turn_o => (1.0, 0.5),
            (Some(turn_x), Some(turn_o)) if turn_x > turn_o => (0.5, 1.0),
            (Some(_), Some(_)) => panic!("I have a tie. I shouldn't be able to have a tie."),
        }
    }
}

const X0: &[char] = &[' ', 'X', 'X', ' ', ' ', 'X', 'X', ' '];
const X1: &[char] = &[' ', ' ', ' ', 'X', 'X', ' ', ' ', ' '];

const O0: &[char] = &[' ', ' ', 'O', 'O', 'O', 'O', ' ', ' '];
const O1: &[char] = &[' ', 'O', 'O', ' ', ' ', 'O', 'O', ' '];

const S: &[char] = &[
    '─', '─', '─', '─', '─', '─', '─', '─', '─', '─', '┼', '─', '─', '─', '─', '─', '─', '─', '─',
    '─', '─', '┼', '─', '─', '─', '─', '─', '─', '─', '─', '─', '─',
];

// TODO: Make this implement one row at a time, and then make consts into strs.
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = [[' '; 32]; 11]; // TODO: Make generic for grid size!

        for row in &mut display {
            row[10] = '│';
            row[21] = '│';
        }

        display[3][..].copy_from_slice(S);
        display[7][..].copy_from_slice(S);

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

#[cfg(test)]
mod test_basic_board_functionality {
    use super::*;

    #[test]
    fn test_get_rows_columns_and_diagonals() {
        let rows_columns_and_diagonals = Board::get_rows_columns_and_diagonals();
        assert_eq!(8, rows_columns_and_diagonals.len());
    }

    #[test]
    #[should_panic(expected = "Invalid position { row: 3, column: 0 }!")]
    fn test_row_out_of_bounds() {
        let mut board = Board::new();
        board.do_turn(Position::new(3, 0), Position::new(3, 0), Token::X);
    }

    #[test]
    #[should_panic(expected = "Invalid position { row: 0, column: 3 }!")]
    fn test_column_out_of_bounds() {
        let mut board = Board::new();
        board.do_turn(Position::new(0, 3), Position::new(0, 3), Token::X);
    }

    #[test]
    #[should_panic(expected = "Invalid move, position { row: 0, column: 0 } already occupied!")]
    fn test_position_occupied() {
        let mut board = Board::new();
        board.do_turn(Position::new(0, 0), Position::new(0, 0), Token::X);
        board.do_turn(Position::new(0, 0), Position::new(0, 0), Token::O);
    }
}

#[cfg(test)]
mod test_searches_and_collapses {
    use super::*;

    fn create_board(spooky_marks: Vec<((usize, usize), (usize, usize))>) -> Board {
        let tokens = [Token::X, Token::O];

        let mut board = Board::new();

        for (turn, (position_1, position_2)) in spooky_marks.into_iter().enumerate() {
            board.do_turn(
                Position::new(position_1.0, position_1.1),
                Position::new(position_2.0, position_2.1),
                tokens[turn % 2],
            );
        }

        board
    }

    #[test]
    fn test_collapse_loop_size_two() {
        let position0 = Position::new(0, 0);
        let position1 = Position::new(1, 1);

        let mut board = Board::new();
        board.do_turn(position0, position1, Token::X);
        board.do_turn(position1, position0, Token::O);

        board.collapse_loop();

        let option1 = board.get_mark(position0) == Some(TurnToken::X(1))
            && board.get_mark(position1) == Some(TurnToken::O(2));
        let option2 = board.get_mark(position0) == Some(TurnToken::O(2))
            && board.get_mark(position1) == Some(TurnToken::X(1));

        assert!(option1 || option2);
    }

    #[test]
    fn test_collapse_loop_size_three() {
        let position0 = Position::new(0, 0);
        let position1 = Position::new(1, 1);
        let position2 = Position::new(2, 2);

        let mut board = Board::new();
        board.do_turn(position0, position1, Token::X);
        board.do_turn(position1, position2, Token::O);
        board.do_turn(position2, position0, Token::X);

        board.collapse_loop();

        let option1 = board.get_mark(position0) == Some(TurnToken::X(1))
            && board.get_mark(position1) == Some(TurnToken::O(2))
            && board.get_mark(position2) == Some(TurnToken::X(3));

        let option2 = board.get_mark(position0) == Some(TurnToken::X(3))
            && board.get_mark(position1) == Some(TurnToken::X(1))
            && board.get_mark(position2) == Some(TurnToken::O(2));

        assert!(option1 || option2);
    }

    #[test]
    fn test_mixed_loop_size_two() {
        let mut board = create_board(vec![
            ((0, 0), (1, 0)),
            ((0, 0), (2, 2)),
            ((0, 1), (2, 1)),
            ((0, 0), (2, 2)),
        ]);

        board.collapse_loop();
    }

    #[test]
    fn test_wikipedia_collapse() {
        let mut board = create_board(vec![
            ((0, 0), (0, 0)),
            ((2, 0), (1, 1)),
            ((0, 1), (1, 1)),
            ((2, 1), (1, 1)),
            ((1, 0), (2, 0)),
            ((2, 2), (2, 2)),
            ((0, 1), (0, 2)),
            ((0, 2), (1, 1)),
        ]);

        board.collapse_loop();

        let (score_x, score_o) = board.get_score();
        assert!(score_x == 0.0 || score_x == 0.5);
        assert_eq!(1.0, score_o);
    }

    #[test]
    fn test_self_fulfilling_loop() {
        let mut board = create_board(vec![
            ((0, 1), (0, 1)),
            ((0, 2), (1, 2)),
            ((0, 0), (1, 0)),
            ((1, 1), (1, 2)),
            ((0, 0), (2, 2)),
            ((2, 0), (0, 2)),
            ((2, 1), (2, 2)),
            ((1, 1), (0, 2)),
        ]);

        board.collapse_loop();

        assert_eq!(None, board.get_mark(Position::new(0, 0)));
        assert_eq!(
            Token::X,
            Token::from(&board.get_mark(Position::new(0, 1)).unwrap())
        );
        assert_eq!(
            Token::O,
            Token::from(&board.get_mark(Position::new(0, 2)).unwrap())
        );

        assert_eq!(None, board.get_mark(Position::new(1, 0)));
        assert_eq!(
            Token::O,
            Token::from(&board.get_mark(Position::new(1, 1)).unwrap())
        );
        assert_eq!(
            Token::O,
            Token::from(&board.get_mark(Position::new(1, 2)).unwrap())
        );

        assert_eq!(
            Token::O,
            Token::from(&board.get_mark(Position::new(2, 0)).unwrap())
        );
        assert_eq!(None, board.get_mark(Position::new(2, 1)));
        assert_eq!(None, board.get_mark(Position::new(2, 2)));

        let (score_x, score_o) = board.get_score();
        assert_eq!(0.0, score_x);
        assert_eq!(1.0, score_o);
    }
}
