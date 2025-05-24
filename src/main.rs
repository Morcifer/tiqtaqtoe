mod board;
mod bot;

use crate::board::{Board, Token};
use crate::bot::random_move;

fn main() {
    println!("Hello! Let's play quantum tic-tac-toe!");

    let mut board = Board::new();

    let markers_order = [Token::X, Token::O];

    // TODO: extract this game loop into a separate game struct
    // And allow seed for bots so you can make random noisy tests.
    while board.turn <= 9 && board.find_winner() == (0.0, 0.0) {
        let token = markers_order[(board.turn - 1) as usize % 2];

        let random_move = random_move(&board, token);

        board.set_spooky_mark(random_move.0, random_move.1, token);

        println!("Board before collapse:");
        print!("{board}");
        board.collapse_loop();
        println!("Board after collapse:");
        print!("{board}");
    }

    println!("Final board:");
    print!("{board}");
    let board_score = board.find_winner();

    let winner = if board_score.0 == 1.0 {
        Some(Token::X)
    } else if board_score.1 == 1.0 {
        Some(Token::O)
    } else {
        None
    };

    match winner {
        Some(winner) => {
            println!(
                "{:?} is the point distribution of {} and {} - {winner} won!",
                board_score,
                Token::X,
                Token::O,
            );
        }
        None => println!("It's a tie!"),
    }

    // let game_moves = [
    //     (Position::new(0, 0), Position::new(0, 0), Token::X),
    //     (Position::new(2, 0), Position::new(1, 1), Token::O),
    //     (Position::new(0, 1), Position::new(1, 1), Token::X),
    //     (Position::new(2, 1), Position::new(1, 1), Token::O),
    //     (Position::new(1, 0), Position::new(2, 0), Token::X),
    //     (Position::new(2, 2), Position::new(2, 2), Token::O),
    //     (Position::new(0, 1), Position::new(0, 2), Token::X),
    //     (Position::new(0, 2), Position::new(1, 1), Token::O),
    // ];

    // for game_move in game_moves {
    //     board.set_spooky_mark(game_move.0, game_move.1, game_move.2);
    //     print!("{board}");
    //
    //     board.collapse_loop();
    //     let board_score = board.find_winner();
    //     if board_score != (0, 0) {
    //         let winner = if board_score.0 == 2 {
    //             Token::X
    //         } else {
    //             Token::O
    //         };
    //
    //         println!(
    //             "{:?} is the point distribution of {} and {} - {winner} won!",
    //             board_score,
    //             Token::X,
    //             Token::O,
    //         );
    //     }
    // }
}
