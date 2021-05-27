#![cfg(test)]

use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

use crate::{
    cell::{Cell, RandomCell},
    game::CellGame,
    game_board::{iter::*, *},
    game_rules::{GameRule, GameRuleIter},
    rgba::RGBA,
};

use crate::cell::mock::*;

#[derive(Clone, Copy, Default)]
pub struct ConwayRule {}
impl GameRule for ConwayRule {
    type Cell = CellConway;
    fn apply(cell: &CellConway, neighbor_iter: NeighborhoodIterator<Self::Cell>) -> CellConway {
        let sum_alive = neighbor_iter
            .map(|c| match c {
                Self::Cell::Dead => 0,
                Self::Cell::Alive => 1,
            })
            .sum();
        match (*cell, sum_alive) {
            (Self::Cell::Alive, 2) => Self::Cell::Alive,
            (_, 3) => Self::Cell::Alive,
            _ => Self::Cell::Dead,
        }
    }
}

// #[derive(Clone)]
// pub struct ConwaysGame {
//     rule: ConwayRule,
//     board: GameBoard<CellConway>,
//     scratch_board: GameBoard<CellConway>,
// }

// impl CellGame for ConwaysGame {
//     type Cell = CellConway;
//     type GameRule = ConwayRule;
//     fn new(width: usize, height: usize) -> ConwaysGame {
//         let board = GameBoard::new(width, height, CellConway::Dead);
//         ConwaysGame {
//             rule: ConwayRule {},
//             board: board.clone(),
//             scratch_board: board,
//         }
//     }
//     fn get_board(&self) -> &GameBoard<Self::Cell> {
//         &self.board
//     }
//     fn step(&mut self) {
//         (&mut self.scratch_board)
//             .into_iter()
//             .zip(self.rule.iter(&self.board))
//             .for_each(|(scratch, new)| *scratch = new);

//         self.board.swap(&mut self.scratch_board);
//     }
//     fn clear(&mut self) {
//         self.board
//             .iter_mut()
//             .for_each(|c: &mut CellConway| *c = CellConway::Dead);
//     }
//     fn dim(&self) -> (usize, usize) {
//         self.board.dim()
//     }
//     fn step_cell(&mut self, x: usize, y: usize) {
//         let cur_cell = self
//             .board
//             .get(x, y)
//             .expect("Indexed outside of valid fields!");
//         let next_cell = match cur_cell {
//             CellConway::Dead => CellConway::Alive,
//             CellConway::Alive => CellConway::Dead,
//         };
//         self.board.set(x, y, next_cell);
//     }
// }

type ConwaysGame = CellGame<CellConway, ConwayRule>;

impl ConwaysGame {
    pub fn new_rand(width: usize, height: usize) -> ConwaysGame {
        let board = GameBoard::new_rand(width, height, CellConway::Dead);
        ConwaysGame {
            rule: ConwayRule {},
            board: board.clone(),
            scratch_board: board,
        }
    }
}

use CellConway::{Alive, Dead};

#[test]
fn test_2x2_conway() {
    let mut board = GameBoard::new(2, 2, Dead);

    board.set(1, 0, Alive);
    board.set(0, 1, Alive);
    board.set(1, 1, Alive);
    let rule = ConwayRule {};
    let v: Vec<CellConway> = rule.iter(&board).collect();
    let v_cmp = vec![Alive; 4];
    assert_eq!(v.to_vec(), v_cmp);
}
#[test]
fn test() {
    test_2x2_conway();
}

#[test]
fn iterate_neighborhood() {
    let (width, height) = (5, 5);
    let mut board = GameBoard::new(width, height, Dead);
    board.set(0, 0, Alive);
    for (i, n) in board.iter_neighbors(0, 0).enumerate() {
        println!("Neighbor {}: {:?}", i, n);
    }
}
#[test]
fn neighbor_counter() {
    let (width, height) = (5, 5);
    let mut board = GameBoard::new(width, height, Dead);
    for c in &mut board {
        *c = Alive;
    }
    let sum: i32 = board
        .iter_neighbors(0, 0)
        .map(|c| match c {
            Dead => 0,
            Alive => 1,
        })
        .sum();
    assert_eq!(sum, 3);
}
#[test]
fn iterate_board() {
    let board = GameBoard::new(10, 10, Dead);
    for n in board.iter() {
        println!("{:?}", n);
    }
}
#[test]
fn iterate_board_and_set_alive() {
    let mut board = GameBoard::new(10, 10, Dead);
    for n in board.iter_mut() {
        *n = Alive;
    }
    for n in board.iter() {
        println!("{:?}", n);
    }
}
