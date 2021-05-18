use crate::game_board::*;

pub trait GameRule {
    fn apply(cell: &Cell, neighbor_iter: NeighborhoodIterator) -> Cell;
    fn iter<'a>(&self, board: &'a GameBoard) -> GameRuleIter<'a> {
        GameRuleIter{board, rule: Self::apply ,x:0,y:0}
    }
}

// consider static dispatch solution
pub struct GameRuleIter<'a> {
    board:&'a GameBoard,
    rule: fn(&Cell, NeighborhoodIterator) -> Cell,
    x:u16,
    y:u16,
}

impl<'a> Iterator for GameRuleIter<'a>{
    type Item = Cell;
    fn next(&mut self) -> Option<Self::Item> {
        let (width, height) = self.board.dim();
        if self.x == width && self.y == height {
            return None;
        }

        let res = (self.rule)(
            &self.board.get(self.x, self.y)?, 
            self.board.iter_neighbors(self.x, self.y)
        );
        self.x += 1;
        if self.x == width {
            self.x = 0;
            self.y += 1;
        }
        Some(res)        
    }
}

pub fn conways_rule(cell: &Cell, neighbor_iter: NeighborhoodIterator) -> Cell {
    let sum_alive = neighbor_iter.map(|c| match c {Cell::Dead => 0, Cell::Alive => 1}).sum();
    match (*cell, sum_alive) {
        (Cell::Alive, 2) => Cell::Alive,
        (_, 3) => Cell::Alive,
        _ => Cell::Dead
    }
}

struct ConwayRule{}
impl GameRule for ConwayRule{
    fn apply(cell: &Cell, neighbor_iter: NeighborhoodIterator) -> Cell {
        let sum_alive = neighbor_iter.map(|c| match c {Cell::Dead => 0, Cell::Alive => 1}).sum();
        match (*cell, sum_alive) {
            (Cell::Alive, 2) => Cell::Alive,
            (_, 3) => Cell::Alive,
            _ => Cell::Dead
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2x2_conway(){
        let mut board = GameBoard::new(2,2);
        board.set(1,0,Cell::Alive);
        board.set(0,1,Cell::Alive);
        board.set(1,1,Cell::Alive);
        let rule = ConwayRule{};
        let v :Vec<Cell> = rule.iter(&board).collect();
        let v_cmp = vec![Cell::Alive; 4];
        assert_eq!(v.to_vec(),v_cmp);
    }
    #[test]
    fn test() {
        test_2x2_conway();
    }
}