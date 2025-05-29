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
}
