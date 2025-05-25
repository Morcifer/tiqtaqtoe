mod board;
mod bot;
mod game;

use crate::bot::RandomBot;
use crate::game::Game;

fn main() {
    println!("Hello! Let's play quantum tic-tac-toe!");

    let random_bot = RandomBot::new(42);
    let mut game = Game::new(random_bot);

    game.play_whole_game();
    game.print_winner();

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
