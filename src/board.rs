use derive_more::Display;

#[derive(Clone, Copy, Display, Eq, PartialEq)]
#[repr(u8)] // TODO: Does this actually do anything?
pub enum Token {
    X,
    O,
}

const BOARD_SIZE: usize = 3;

pub struct Board {
    board: [[Option<Token>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn set_token(&mut self, row: usize, column: usize, token: Token) {
        // TODO: validate un-occupied.
        self.board[row][column] = Some(token);
    }

    // TODO: Add tests for a bunch of cases
    pub fn find_winner(&self) -> (bool, Option<Token>) {
        // TODO: Extract into constants in constructor!
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

        // Not determined != Both win
        let x_wins = rows_columns_and_diagonals.iter().any(|v| {
            v.iter().all(|(row_index, column_index)| {
                self.board[*row_index][*column_index] == Some(Token::X)
            })
        });
        let o_wins = rows_columns_and_diagonals.iter().any(|v| {
            v.iter().all(|(row_index, column_index)| {
                self.board[*row_index][*column_index] == Some(Token::O)
            })
        });

        match (x_wins, o_wins) {
            (false, false) => (false, None),
            (true, false) => (true, Some(Token::X)),
            (false, true) => (true, Some(Token::O)),
            (true, true) => (true, None),
        }
    }
}
