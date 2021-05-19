use crate::game_board::*;
use crate::game_rules::*;

use rayon::prelude::*;


#[derive(Clone)]
pub struct ConwaysGame {
    rule: ConwayRule,
    board: GameBoard<CellConway>,
    scratch_board: GameBoard<CellConway>,
}

impl ConwaysGame {
    pub fn new(width: u16, height: u16) -> ConwaysGame {
        let board = GameBoard::new(width,height, CellConway::Dead);
        ConwaysGame{rule: ConwayRule{}, board: board.clone(), scratch_board: board}
    }
    pub fn new_rand(width: u16, height: u16) -> ConwaysGame {
        let board = GameBoard::new_rand(width, height, CellConway::Dead);
        ConwaysGame{rule: ConwayRule{}, board: board.clone(), scratch_board: board}
    }
    pub fn get_board(&self) -> &GameBoard<CellConway> {
        &self.board
    }
    pub fn evolve(&mut self) -> &GameBoard<CellConway> {

        // let _ = (&mut self.scratch_board).into_iter()
        // .zip(self.rule.iter(&self.board))
        // // .par_bridge()
        // .map(|(scratch,new)|{*scratch = new;});

        // (&mut self.scratch_board).into_par_iter()
        // .zip(self.rule.iter(&self.board))
        // .par_bridge()
        // .for_each(|(scratch, new)| *scratch = new);

        (&mut self.scratch_board).into_iter()
        .zip(self.rule.iter(&self.board))
        .for_each(|(scratch,new)| *scratch = new);

        self.board.swap(&mut self.scratch_board);
        &self.board
    }
    pub fn dim(&self) -> (u16,u16) {
        self.board.dim()
    }
}