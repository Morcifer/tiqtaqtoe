use rand::prelude::IndexedRandom;

use crate::board::{Board, Position, SpookyMark, Token};

pub fn random_move(board: &Board, _token: Token) -> (Position, Position) {
    let mut possible_moves = vec![];

    for position_1 in &board.positions {
        if board.get_mark(*position_1).is_some() {
            continue;
        }

        for position_2 in &board.positions {
            if board.get_mark(*position_2).is_some() {
                continue;
            }

            if board.spooky_marks.iter().any(|SpookyMark(p1, p2, _)| {
                position_1 == position_2 && (position_1 == p1 || position_1 == p2)
            }) {
                continue;
            }

            possible_moves.push((*position_1, *position_2));
        }
    }

    *possible_moves.choose(&mut rand::rng()).unwrap() // TODO: Put in struct!
}
