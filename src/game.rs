use crate::game_board::*;
use crate::game_rules::*;


#[derive(Clone)]
pub struct ConwaysGame {
    board: GameBoard,
    scratch_board: GameBoard,
}

impl GameRule for ConwaysGame {
    fn apply(cell: &Cell, neighbor_iter: NeighborhoodIterator) -> Cell {
        conways_rule(cell, neighbor_iter)
    }
}

impl ConwaysGame {
    pub fn new(width: u16, height: u16) -> ConwaysGame {
        let board = GameBoard::new(width,height);
        ConwaysGame{ board: board.clone(), scratch_board: board}
    }
    pub fn new_rand(width: u16, height: u16) -> ConwaysGame {
        let board = GameBoard::new_rand(width, height);
        ConwaysGame{board: board.clone(), scratch_board: board}
    }
    pub fn get_board(&self) -> &GameBoard {
        &self.board
    }
    pub fn evolve(&mut self) -> &GameBoard {
        let (width, height) = self.board.dim();
        self.scratch_board = GameBoard::from_slice(&mut self.iter(&self.board).collect::<Vec<Cell>>(), width, height).unwrap();
        self.board.swap(&mut self.scratch_board);
        &self.board
    }
    pub fn dim(&self) -> (u16,u16) {
        self.board.dim()
    }
}