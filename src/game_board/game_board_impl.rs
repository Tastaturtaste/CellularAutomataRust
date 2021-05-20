use super::{detail::build_neighbor_lookup, iter::*, GameBoard};
use crate::cell::*;

impl<'a, C: Cell> GameBoard<C> {
    pub fn new(width: usize, height: usize, border_cell: C) -> Self {
        let inner_width = width + 2;
        let inner_height = height + 2;
        let num_nodes = inner_width * inner_height;
        Self {
            inner_width,
            inner_height,
            width,
            height,
            cells: vec![border_cell.clone(); num_nodes],
            neighbor_lookup: build_neighbor_lookup(inner_width),
        }
    }
    pub fn new_rand(width: usize, height: usize) -> Self {
        let inner_width = width + 2;
        let inner_height = height + 2;
        let num_nodes = inner_width * inner_height;
        let mut cells = Vec::new();
        cells.reserve(num_nodes);
        for _ in 0..(num_nodes) {
            cells.push(Cell::new_rand());
        }
        Self {
            inner_width,
            inner_height,
            width,
            height,
            cells,
            neighbor_lookup: build_neighbor_lookup(inner_width),
        }
    }
    pub fn iter(&self) -> GameBoardIterator<'_, C> {
        //let iter =(self.inner_width + 1..self.cells.len() - self.inner_width).into_iter().filter(move |&i| self.check_index(i)).map(move |i| unsafe {self.cells.get_unchecked(i)});
        // GameBoardIterator {
        let offset = self.inner_width + 1;
        let end = self.cells.len() - self.inner_width - 1;
        // }
        GameBoardIterator::new(&self, offset, end)
        //self.cells.iter().enumerate().filter(|(i,c)| self.check_index(*i) )
    }
    pub fn iter_mut(&mut self) -> GameBoardMutIterator<'_, C> {
        let offset = self.inner_width + 1;
        let end = self.cells.len() - self.inner_width - 1;
        GameBoardMutIterator::new(self, offset, end)
    }
    pub fn iter_neighbors(&self, x: usize, y: usize) -> NeighborhoodIterator<C> {
        // Allow only indexing into the inner parts of the board
        let (x, y) = self.coord_public_to_inner(x, y);
        self.iter_neighbors_inner(x, y)
    }
    pub fn iter_neighbors_index(&self, i: usize) -> NeighborhoodIterator<C> {
        // Allow only indexing into the inner parts of the board
        let i = self.index_public_to_inner(i);
        self.iter_neighbors_index_inner(i)
    }
    pub fn iter_local_groups(&self) -> LocalGroupIterator<C> {
        LocalGroupIterator::new(&self, self.index_public_to_inner(0))
    }
    pub fn get(self: &Self, x: usize, y: usize) -> Option<&C> {
        let (x, y) = self.coord_public_to_inner(x, y);
        self.cells.get(self.coord_to_index_inner(x, y))
    }
    pub fn get_mut(self: &mut Self, x: usize, y: usize) -> Option<&mut C> {
        let (x, y) = self.coord_public_to_inner(x, y);
        let index = self.coord_to_index_inner(x, y);
        self.cells.get_mut(index)
    }
    pub fn set(&mut self, x: usize, y: usize, cell: C) {
        let (x, y) = self.coord_public_to_inner(x, y);
        let index = self.coord_to_index_inner(x, y);
        self.cells[index] = cell;
    }
    #[inline]
    pub fn dim(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    pub fn swap(&mut self, other: &mut Self) {
        if !(self.dim() == other.dim()) {
            panic!("GameBoards cannot swap because of unequal dimension!");
        }
        std::mem::swap(&mut self.cells, &mut other.cells);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::cell::CellConway::*;
    #[test]
    fn iterate_neighborhood() {
        let mut board = GameBoard::new(10, 10, Dead);
        board.set(0, 0, Alive);
        for n in board.iter_neighbors(0, 0) {
            println!("{:?}", n);
        }
    }
    #[test]
    fn neighbor_count_corner() {
        let mut board = GameBoard::new(10, 10, Dead);
        for c in &mut board {
            *c = Alive;
        }
        let sum: i32 = board
            .iter_neighbors(0, 0)
            .map(|c| match c {
                Dead => 0,
                Alive => 1,
            })
            .sum();
        assert_eq!(sum, 3);
    }
    #[test]
    fn iterate_board() {
        let board = GameBoard::new(10, 10, Dead);
        for n in board.iter() {
            println!("{:?}", n);
        }
    }
    #[test]
    fn iterate_board_and_set_alive() {
        let mut board = GameBoard::new(10, 10, Dead);
        for n in board.iter_mut() {
            *n = Alive;
        }
        for n in board.iter() {
            println!("{:?}", n);
        }
    }
}
