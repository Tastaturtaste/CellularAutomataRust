use crate::cell::*;
use crate::game_board::*;
use crate::game_rules::*;

pub trait CellGame {
    type Cell: Cell;
    type GameRule: GameRule;
    fn get_board(&self) -> &GameBoard<Self::Cell>;
    fn step(&mut self);
    fn clear(&mut self);
    fn dim(&self) -> (usize, usize);
    fn step_cell(&mut self, x: usize, y: usize);
}

#[derive(Clone)]
pub struct ConwaysGame {
    rule: ConwayRule,
    board: GameBoard<CellConway>,
    scratch_board: GameBoard<CellConway>,
}

impl CellGame for ConwaysGame {
    type Cell = CellConway;
    type GameRule = ConwayRule;
    fn get_board(&self) -> &GameBoard<Self::Cell> {
        &self.board
    }
    fn step(&mut self) {
        (&mut self.scratch_board)
            .into_iter()
            .zip(self.rule.iter(&self.board))
            .for_each(|(scratch, new)| *scratch = new);

        self.board.swap(&mut self.scratch_board);
    }
    fn clear(&mut self) {
        self.board
            .iter_mut()
            .for_each(|c: &mut CellConway| *c = CellConway::Dead);
    }
    fn dim(&self) -> (usize, usize) {
        self.board.dim()
    }
    fn step_cell(&mut self, x: usize, y: usize) {
        let cur_cell = self
            .board
            .get(x, y)
            .expect("Indexed outside of valid fields!");
        let next_cell = match cur_cell {
            CellConway::Dead => CellConway::Alive,
            CellConway::Alive => CellConway::Dead,
        };
        self.board.set(x, y, next_cell);
    }
}

impl ConwaysGame {
    pub fn new(width: usize, height: usize) -> ConwaysGame {
        let board = GameBoard::new(width, height, CellConway::Dead);
        ConwaysGame {
            rule: ConwayRule {},
            board: board.clone(),
            scratch_board: board,
        }
    }
    pub fn new_rand(width: usize, height: usize) -> ConwaysGame {
        let board = GameBoard::new_rand(width, height, CellConway::Dead);
        ConwaysGame {
            rule: ConwayRule {},
            board: board.clone(),
            scratch_board: board,
        }
    }
}
