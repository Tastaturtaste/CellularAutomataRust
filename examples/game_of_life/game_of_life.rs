use crate::cell::CellConway;
pub use cell_engine_rs::game::{
    traits::{CellGame as CellGameTrait, RandCellGame as RandCellGameTrait},
    CellGame,
};
use cell_engine_rs::{game_board::iter::*, game_rules::*};

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

pub type ConwaysGame = CellGame<CellConway, ConwayRule>;
