use crate::cell::WireCell;
pub use cell_engine::game::traits::CellGame as CellGameTrait;
use cell_engine::{
    cell::*,
    game::CellGame,
    game_board::{iter::*, GameBoard},
    game_rules::*,
};

#[derive(Default, Clone, Copy)]
pub struct WireworldRule {}

impl GameRule for WireworldRule {
    type Cell = WireCell;

    fn apply(cell: &Self::Cell, neighbor_iter: NeighborhoodIterator<Self::Cell>) -> Self::Cell {
        match *cell {
            WireCell::Off => WireCell::Off,
            WireCell::Wire => {
                let sum = neighbor_iter
                    .filter(|&cell| *cell == WireCell::ElectronHead)
                    .count();
                if sum >= 1 && sum <= 2 {
                    WireCell::ElectronHead
                } else {
                    WireCell::Wire
                }
            }
            WireCell::ElectronHead => WireCell::ElectronTail,
            WireCell::ElectronTail => WireCell::Wire,
        }
    }
}

pub type WireGame = CellGame<WireCell, WireworldRule>;
