use rand::prelude::IndexedRandom;

use crate::board::{Board, Position, SpookyMark};

pub fn random_move(board: &Board) -> (Position, Position) {
    // TODO: There's a missing action - if there's a spooky X mark,
    // O can do the same to force a collapse!
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
                [p1, p2] == [position_1, position_2] || [p1, p2] == [position_2, position_1]
            }) {
                continue;
            }

            possible_moves.push((*position_1, *position_2));
        }
    }

    *possible_moves.choose(&mut rand::rng()).unwrap() // TODO: Put in struct!
}
