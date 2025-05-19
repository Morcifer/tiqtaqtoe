use derive_more::Display;

#[derive(Clone, Copy, Display, Eq, PartialEq)]
#[repr(u8)] // TODO: Does this actually do anything?
pub enum Token {
    X,
    O,
}

#[derive(Clone, Copy, Display, Eq, PartialEq)]
#[repr(u8)] // TODO: Does this actually do anything?
pub enum TurnToken {
    X(u8),
    O(u8),
}

const BOARD_SIZE: usize = 3;

pub struct Board {
    board: [[Option<TurnToken>; BOARD_SIZE]; BOARD_SIZE],
    turn: u8,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            turn: 0,
        }
    }

    pub fn set_token(&mut self, row: usize, column: usize, token: Token) {
        // TODO: validate un-occupied.
        match token {
            Token::X => self.board[row][column] = Some(TurnToken::X(self.turn)),
            Token::O => self.board[row][column] = Some(TurnToken::O(self.turn)),
        }

        self.turn += 1;
    }

    // TODO: Add tests for a bunch of cases
    pub fn find_winner(&self) -> (usize, usize) {
        // Points X, points O
        // TODO: Extract into list made in constructor!
        let mut rows_columns_and_diagonals: Vec<Vec<(usize, usize)>> = vec![];

        for row_index in 0..BOARD_SIZE {
            rows_columns_and_diagonals.push(
                (0..BOARD_SIZE)
                    .map(|column_index| (row_index, column_index))
                    .collect(),
            );
        }

        for column_index in 0..BOARD_SIZE {
            rows_columns_and_diagonals.push(
                (0..BOARD_SIZE)
                    .map(|row_index| (row_index, column_index))
                    .collect(),
            );
        }

        rows_columns_and_diagonals.push(
            (0..BOARD_SIZE)
                .map(|row_index| (row_index, row_index))
                .collect(),
        );
        rows_columns_and_diagonals.push(
            (0..BOARD_SIZE)
                .map(|row_index| (row_index, BOARD_SIZE - row_index - 1))
                .collect(),
        );

        // TODO: extract to reduce repeated code.
        let x_wins = rows_columns_and_diagonals
            .iter()
            .filter_map(|v| {
                let max_turn = v
                    .iter()
                    .map(
                        |(row_index, column_index)| match self.board[*row_index][*column_index] {
                            Some(TurnToken::X(turn)) => turn,
                            _ => u8::MAX,
                        },
                    )
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
                    .map(
                        |(row_index, column_index)| match self.board[*row_index][*column_index] {
                            Some(TurnToken::O(turn)) => turn,
                            _ => u8::MAX,
                        },
                    )
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
