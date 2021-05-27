use crate::cell::*;
use crate::game_board::*;
use crate::game_rules::*;

pub mod traits {
    use super::*;
    pub trait CellGame {
        type Cell: Cell;
        type GameRule: GameRule;
        fn get_board(&self) -> &GameBoard<Self::Cell>;
        fn step(&mut self);
        fn clear(&mut self, clear_cell: Self::Cell);
        fn dim(&self) -> (usize, usize);
        fn next_cell(&mut self, x: usize, y: usize);
        fn new(width: usize, height: usize, border_cell: Self::Cell) -> Self;
    }
    pub trait RandCellGame: CellGame {
        fn new_rand(width: usize, height: usize, border_cell: Self::Cell) -> Self;
    }
}

pub struct CellGame<C: Cell, R: GameRule<Cell = C>> {
    pub(crate) rule: R,
    pub(crate) board: GameBoard<C>,
    pub(crate) scratch_board: GameBoard<C>,
}

impl<C: RandomCell, R: GameRule<Cell = C>> traits::RandCellGame for CellGame<C, R> {
    fn new_rand(width: usize, height: usize, border_cell: Self::Cell) -> Self {
        let board = GameBoard::new_rand(width, height, border_cell);
        Self {
            rule: R::default(),
            board: board.clone(),
            scratch_board: board,
        }
    }
}

impl<C: Cell, R: GameRule<Cell = C>> traits::CellGame for CellGame<C, R> {
    type Cell = C;

    type GameRule = R;

    fn get_board(&self) -> &GameBoard<Self::Cell> {
        &self.board
    }
    fn step(&mut self) {
        self.scratch_board
            .iter_mut()
            .zip(self.rule.iter(&self.board))
            .for_each(|(scratch_cell, cell)| *scratch_cell = cell);
        self.board.swap(&mut self.scratch_board);
    }
    fn clear(&mut self, clear_cell: Self::Cell) {
        self.board.iter_mut().for_each(|cell| *cell = clear_cell);
    }
    fn dim(&self) -> (usize, usize) {
        self.board.dim()
    }
    fn next_cell(&mut self, x: usize, y: usize) {
        let cur_cell = self.board.get(x, y).expect("Index out of bounds!");
        let next_cell = cur_cell.next();
        // let neighbor_iter = self.board.iter_neighbors(x, y);
        // let next_cell = Self::GameRule::apply(cur_cell, neighbor_iter);
        self.board.set(x, y, next_cell);
    }
    fn new(width: usize, height: usize, border_cell: Self::Cell) -> Self {
        let board = GameBoard::<C>::new(width, height, border_cell);
        Self {
            rule: R::default(),
            board: board.clone(),
            scratch_board: board,
        }
    }
}
