pub mod iter;
use std::usize;

use crate::cell::*;
use iter::*;

#[derive(Default, Debug, Clone)]
pub struct GameBoard<C: Cell> {
    outer_width: usize,
    outer_height: usize,
    width: usize,
    height: usize,
    cells: Vec<C>,
    neighbor_lookup: [isize; 8],
}

/// Public impl
impl<'a, C: Cell> GameBoard<C> {
    pub fn new(width: usize, height: usize, border_cell: C) -> Self {
        let outer_width = width + 2;
        let outer_height = height + 2;
        let num_nodes = outer_width * outer_height;
        Self {
            outer_width,
            outer_height,
            width,
            height,
            cells: vec![border_cell.clone(); num_nodes],
            neighbor_lookup: build_neighbor_lookup(outer_width),
        }
    }

    pub fn new_rand(width: usize, height: usize, border_cell: C) -> Self {
        let outer_width = width + 2;
        let outer_height = height + 2;
        let num_nodes = outer_width * outer_height;
        let mut cells = Vec::new();

        cells.reserve(num_nodes);
        for i in 0..(num_nodes) {
            let (x, y) = index_to_coord(i, outer_width);
            let is_border = check_border(x, y, outer_width, outer_height);
            if is_border {
                cells.push(border_cell)
            } else {
                cells.push(Cell::new_rand());
            }
        }
        Self {
            outer_width,
            outer_height,
            width,
            height,
            cells,
            neighbor_lookup: build_neighbor_lookup(outer_width),
        }
    }
    pub fn iter(&self) -> GameBoardIterator<'_, C> {
        //let iter =(self.outer_width + 1..self.cells.len() - self.outer_width).into_iter().filter(move |&i| self.check_index(i)).map(move |i| unsafe {self.cells.get_unchecked(i)});
        // GameBoardIterator {
        let offset = self.outer_width + 1;
        let end = self.cells.len() - self.outer_width - 1;
        // }
        GameBoardIterator::new(&self, offset, end)
        //self.cells.iter().enumerate().filter(|(i,c)| self.check_index(*i) )
    }
    pub fn iter_mut(&mut self) -> GameBoardMutIterator<'_, C> {
        let offset = self.outer_width + 1;
        let end = self.cells.len() - self.outer_width - 1;
        GameBoardMutIterator::new(self, offset, end)
    }
    pub fn iter_neighbors(&self, x: usize, y: usize) -> NeighborhoodIterator<C> {
        let (x, y) = coord_to_outer(x, y);
        // Allow only indexing into the inner parts of the board
        debug_assert!(!check_border(x, y, self.outer_width, self.outer_height), "Coordinates index a border region!");
        NeighborhoodIterator::new(
            &self,
            coord_to_index(x, y, self.outer_width) as isize,
            self.neighbor_lookup.iter(),
        )
        // self.iter_neighbors_inner(x, y)
    }
    pub fn iter_neighbors_index(&self, i: usize) -> NeighborhoodIterator<C> {
        // Allow only indexing into the inner parts of the board
        let i = self.index_to_outer(i);
        self.iter_neighbors_index_outer(i)
    }
    pub fn iter_local_groups(&self) -> LocalGroupIterator<C> {
        LocalGroupIterator::new(&self,self.index_to_outer(0))
    }
    pub fn get(self: &Self, x: usize, y: usize) -> Option<&C> {
        let(x,y) = coord_to_outer(x, y);
        let i = coord_to_index(x, y, self.outer_width);
        self.cells.get(i)
    }
    pub fn get_mut(self: &mut Self, x: usize, y: usize) -> Option<&mut C> {
        let(x,y) = coord_to_outer(x, y);
        let i = coord_to_index(x, y, self.outer_width);
        self.cells.get_mut(i)
    }
    pub fn set(&mut self, x: usize, y: usize, cell: C) {
        let(x,y) = coord_to_outer(x, y);
        let i = coord_to_index(x, y, self.outer_width);
        self.cells[i] = cell;
    }
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

/// Private impl
impl<'a, C: Cell> GameBoard<C> {
    fn index_from_outer(&self, i: usize) -> usize {
        debug_assert!({
            let (x, y) = index_to_coord(i, self.outer_width);
            !check_border(x, y, self.outer_width, self.outer_height)
        });
        let i = i as isize;
        let i = i
            - self.outer_width as isize // Subtract upper border
            - 2*(i / self.outer_width as isize) // Subtract right border cell of row above and left border cell of current row up to current one
            + 1; // Correct previous statement for the right border cell of the upper row, which was subtracted as a whole 
        i as usize
    }
    fn index_to_outer(&self, i: usize) -> usize {
        debug_assert!({
            let (x, y) = index_to_coord(i, self.width);
            x < self.width && y < self.height // Detect if the provided index would be in a border region
        });
        let i = i as isize;
        let i = i 
        + self.outer_width as isize // Add upper border row
        + 2*(i / self.width as isize) // Add right border cell of row above and left border cell of current row up to current one
        + 1; // Correct previous statement for the right border cell of the upper row, which is not included in the previous calculation
        i as usize
    }
    fn iter_neighbors_outer(&self, x: usize, y: usize) -> NeighborhoodIterator<C> {
    //Allow only indexing into the inner parts of the board
        debug_assert!(!check_border(x, y, self.outer_width, self.outer_height));
        NeighborhoodIterator::new(
            self,
            coord_to_index(x, y, self.outer_width) as isize,
            self.neighbor_lookup.iter(),
        )
    }

    /// Argument has to be a valid index into the inner part of the board
    fn iter_neighbors_index_outer(&self, i: usize) -> NeighborhoodIterator<C> {
        // Allow only indexing into the inner parts of the board
        debug_assert!({
            let (x,y) = index_to_coord(i, self.outer_width);
            !check_border(x, y, self.outer_width, self.outer_height)
        });
        NeighborhoodIterator::new(self, i as isize, self.neighbor_lookup.iter())
    }
    unsafe fn get_unchecked_outer(self: &Self, x: usize, y: usize) -> &C {
        let (x,y) = coord_to_outer(x, y);
        debug_assert!(check_border(x, y, self.outer_width, self.outer_height));
        let i = coord_to_index(x,y,self.outer_width);
        self.cells.get_unchecked(i)
    }
    fn get_index_outer(self: &Self, i: usize) -> Option<&C> {
        self.cells.get(i)
    }
    unsafe fn get_unchecked_index_outer(self: &Self, i: usize) -> &C {
        self.cells.get_unchecked(i)
    }
    unsafe fn get_unchecked_mut_index_outer(self: &mut Self, i: usize) -> &mut C {
        self.cells.get_unchecked_mut(i)
    }
    fn to_vec(self) -> Vec<C> {
        self.cells
    }
}

fn build_neighbor_lookup(outer_width: usize) -> [isize; 8] {
    [
        -(outer_width as isize) - 1,
        -(outer_width as isize),
        -(outer_width as isize) + 1,
        -1,
        1,
        (outer_width as isize) - 1,
        (outer_width as isize),
        (outer_width as isize) + 1,
    ]
}

fn index_to_coord(i: usize, width: usize) -> (usize, usize) {
    let x = i % width;
    let y = i / width;
    (x, y)
}
fn coord_to_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}
fn check_not_border(x: usize, y: usize, width: usize, height: usize) -> bool {
    x > 0 && x < (width - 1) && y > 0 && y < (height - 1)
}
fn check_border(x: usize, y: usize, width: usize, height: usize) -> bool {
    !check_not_border(x, y, width, height)
}
fn coord_to_outer(x: usize, y: usize) -> (usize, usize) {
    (x + 1, y + 1)
}
fn coord_from_outer(x: usize, y: usize) -> (usize, usize) {
    (x - 1, y - 1)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::cell::CellConway::*;


    #[test]
    fn test_index_roundtrip(){
        let (width,height) = (5,5);
        let board = GameBoard::new(width,height, CellConway::Dead);
        for i_inner in 0..width*height{
            let i_outer = board.index_to_outer(i_inner);
            assert!(i_inner == board.index_from_outer(i_outer));
        }
    }
    #[test]
    fn test_coord_roundtrip() {
        let (width,height) = (5,5);
        for x_inner in 0..width {
            for y_inner in 0..height {
                let (x_outer, y_outer) = coord_to_outer(x_inner, y_inner);
                let (x_back, y_back) = coord_from_outer(x_outer, y_outer);
                assert!(x_back == x_inner && y_back == y_inner);
            }
        }
    }

    #[test]
    fn iterate_neighborhood() {
        let (width, height) = (5,5);
        let mut board = GameBoard::new(width, height, Dead);
        board.set(0, 0, Alive);
        for (i,n) in board.iter_neighbors(0, 0).enumerate() {
            println!("Neighbor {}: {:?}", i,n);
        }
    }
    #[test]
    fn neighbor_counter() {
        let (width, height) = (5,5);
        let mut board = GameBoard::new(width, height, Dead);
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
