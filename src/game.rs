use crate::board::{Board, Token};
use crate::bot::Bot;

pub struct Game<B>
where
    B: Bot,
{
    bot: B,
    board: Board,
    tokens: [Token; 2],
}

impl<B> Game<B>
where
    B: Bot,
{
    pub fn new(bot: B) -> Self {
        Self {
            bot,
            board: Board::new(),
            tokens: [Token::X, Token::O],
        }
    }

    pub fn play_turn(&mut self) {
        let token = self.tokens[(self.board.turn - 1) as usize % 2];

        let random_move = self.bot.get_next_move(&self.board, token);

        self.board
            .set_spooky_mark(random_move.0, random_move.1, token);

        println!("Board before collapse:");
        print!("{}", self.board);
        self.board.collapse_loop();
        println!("Board after collapse:");
        print!("{}", self.board);
    }

    pub fn play_whole_game(&mut self) {
        while self.board.turn <= 9 && self.board.get_score() == (0.0, 0.0) {
            self.play_turn();
        }
    }

    pub fn print_winner(&self) {
        println!("Final board:");
        print!("{}", self.board);
        let board_score = self.board.get_score();

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
    }
}

#[cfg(test)]
mod random_bot_game_tests {
    use rstest::rstest;

    use super::*;

    use crate::bot::RandomBot;

    #[rstest]
    fn game_test(#[values(0, 1, 2, 13, 42, 100, 31415)] seed: u64) {
        let random_bot = RandomBot::new(seed);
        let mut game = Game::new(random_bot);

        // As long as it doesn't crash, we're probably fine.
        game.play_whole_game();
    }
}
