use crate::cell::*;
use crate::game_board::iter::*;
use crate::game_board::*;

pub trait GameRule: Copy + Default {
    type Cell: Cell;
    /// The implementation of this function should apply the game rule that is represented with this struct for cell based on all eight neigboring cells.
    fn apply(cell: &Self::Cell, neighbor_iter: NeighborhoodIterator<Self::Cell>) -> Self::Cell;
    fn iter<'a>(&self, board: &'a GameBoard<Self::Cell>) -> GameRuleIter<'a, Self> {
        GameRuleIter {
            rule: Self::apply,
            local_iter: board.iter_local_groups(),
            // board,
            // x: 0,
            // y: 0,
        }
    }
}

pub struct GameRuleIter<'a, Rule: GameRule + ?Sized> {
    rule: fn(&Rule::Cell, NeighborhoodIterator<Rule::Cell>) -> Rule::Cell,
    local_iter: LocalGroupIterator<'a, Rule::Cell>,
    // board: &'a GameBoard<Rule::Cell>,
    // x: usize,
    // y: usize,
}

impl<'a, Rule> Iterator for GameRuleIter<'a, Rule>
where
    Rule: GameRule,
{
    type Item = Rule::Cell;
    fn next(&mut self) -> Option<Self::Item> {
        let (c, n) = self.local_iter.next()?;
        Some((self.rule)(c, n))
    }
}
