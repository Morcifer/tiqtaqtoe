mod board;

use crate::board::{Board, Token};

fn main() {
    println!("Hello, world! Let's play!");

    let mut board = Board::new();

    board.set_token(1, 0, Token::X);
    board.set_token(1, 1, Token::X);
    board.set_token(1, 2, Token::X);

    board.set_token(0, 0, Token::O);
    board.set_token(0, 1, Token::O);
    board.set_token(0, 2, Token::O);

    println!(
        "{:?} is the point distribution of {} and {}!",
        board.find_winner(),
        Token::X,
        Token::O
    );
}
