mod board;

use crate::board::{Board, Token};

fn main() {
    println!("Hello, world! Let's play!");

    let mut board = Board::new();

    board.set_token(0, 0, Token::X);
    board.set_token(1, 1, Token::X);
    board.set_token(2, 2, Token::X);

    println!("{} is the winner!", board.find_winner().1.unwrap());
}
