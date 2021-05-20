mod detail;
pub mod game_board_impl;
pub mod iter;

use crate::cell::Cell;
pub use game_board_impl::*;
pub use iter::*;
#[derive(Default, Debug, Clone)]
pub struct GameBoard<C: Cell> {
    inner_width: usize,
    inner_height: usize,
    width: usize,
    height: usize,
    cells: Vec<C>,
    neighbor_lookup: [isize; 8],
}
