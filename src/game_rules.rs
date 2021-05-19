use crate::game_board::*;

pub trait GameRule: Copy {
    type Cell: Cell;
    fn apply(cell: &Self::Cell, neighbor_iter: NeighborhoodIterator<Self::Cell>) -> Self::Cell;
    fn iter<'a>(&self, board: &'a GameBoard<Self::Cell>) -> GameRuleIter<'a,Self> {
        GameRuleIter{board, rule: Self::apply ,x:0,y:0}
    }
    // fn par_iter<'a>(&self, board: &'a GameBoard<Self::Cell>) -> GameRuleIter<'a,Self>{

    // }
}

pub struct GameRuleIter<'a, Rule: GameRule + ?Sized> {
    board:&'a GameBoard<Rule::Cell>,
    rule: fn(&Rule::Cell, NeighborhoodIterator<Rule::Cell>) -> Rule::Cell,
    x:u16,
    y:u16,
}

impl<'a, Rule> Iterator for GameRuleIter<'a, Rule>
where Rule: GameRule
{
    type Item = Rule::Cell;
    fn next(&mut self) -> Option<Self::Item> {
        let (width, height) = self.board.dim();
        if self.x == width && self.y == height {
            return None;
        }

        let res: Self::Item = (self.rule)(
            self.board.get(self.x, self.y)?, 
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


#[derive(Clone, Copy)]
pub struct ConwayRule{}
impl GameRule for ConwayRule {
    type Cell = CellConway;
    fn apply(cell: &CellConway, neighbor_iter: NeighborhoodIterator<Self::Cell>) -> CellConway {
        let sum_alive = neighbor_iter.map(|c| match c {Self::Cell::Dead => 0, Self::Cell::Alive => 1}).sum();
        match (*cell, sum_alive) {
            (Self::Cell::Alive, 2) => Self::Cell::Alive,
            (_, 3) => Self::Cell::Alive,
            _ => Self::Cell::Dead
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use CellConway::*;

    #[test]
    fn test_2x2_conway(){
        let mut board = GameBoard::new(2,2, Dead);
        board.set(1,0,Alive);
        board.set(0,1,Alive);
        board.set(1,1,Alive);
        let rule = ConwayRule{};
        let v :Vec<CellConway> = rule.iter(&board).collect();
        let v_cmp = vec![Alive; 4];
        assert_eq!(v.to_vec(),v_cmp);
    }
    #[test]
    fn test() {
        test_2x2_conway();
    }
}