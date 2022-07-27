pub mod iter;
use std::usize;

use crate::cell::*;
use iter::*;

#[derive(Default, Debug, Clone)]
pub struct GameBoard<C: Cell> {
    outer_width: usize, 
    // outer_height can be calculated if needed: cells.len() / outer_width 
    width: usize,
    height: usize,
    cells: Vec<C>,
    neighbor_lookup: [isize; 8],
}

/// Public impl
impl<'a, C: Cell> GameBoard<C> {

    /// Construct a new GameBoard with the given width and height and the passed cell variant 
    /// as the cell lining the border of the GameBoard. The border cell is fixed and cannot be updated.
    pub fn new(width: usize, height: usize, border_cell: C) -> Self {
        let outer_width = width + 2;
        let outer_height = height + 2;
        let num_nodes = outer_width * outer_height;
        Self {
            outer_width,

            width,
            height,
            cells: vec![border_cell; num_nodes],
            neighbor_lookup: build_neighbor_lookup(outer_width),
        }
    }
    /// Construct a GameBoardIterator to iterate over the GameBoard cells.
    pub fn iter(&self) -> GameBoardIterator<'_, C> {
        let offset = self.outer_width + 1;
        let end = self.cells.len() - self.outer_width - 1;
        GameBoardIterator::new(self, offset, end)
    }
    /// Construct a mutable GameBoardIterator to iterate over the GameBoard cells.
    pub fn iter_mut(&mut self) -> GameBoardMutIterator<'_, C> {
        let offset = self.outer_width + 1;
        let end = self.cells.len() - self.outer_width - 1;
        GameBoardMutIterator::new(self, offset, end)
    }
    /// Construct a NeighborhoodIterator to iterate over the cells surounding the cell at the given coordinates.
    pub fn iter_neighbors(&self, x: usize, y: usize) -> NeighborhoodIterator<C> {
        // Allow only indexing into the inner parts of the board
        assert!(check_not_border(x, y, self.width, self.height));
        let (x, y) = coord_inner_to_outer(x, y);
        NeighborhoodIterator::new(
            self,
            coord_to_index(x, y, self.outer_width) as isize,
            self.neighbor_lookup.iter(),
        )
    }
    /// Construct a NeighborhoodIterator to iterate over the cells surounding the cell at position given by the index. The index 
    pub fn iter_neighbors_index(&self, i: usize) -> NeighborhoodIterator<C> {
        // Allow only indexing into the inner parts of the board
        let (x,y) = index_to_coord(i, self.width);
        assert!(check_not_border(x, y, self.width, self.height));
        let i = self.index_inner_to_outer(i);
        self.iter_neighbors_index_outer(i)
    }
    /// Construct a LocalGroupIterator to iterate over all neighborhoods on the GameBoard.
    pub fn iter_local_groups(&self) -> LocalGroupIterator<C> {
        LocalGroupIterator::new(self,self.index_inner_to_outer(0))
    }
    /// Get a shared borrow of the cell at the given coordinate
    pub fn get(&self, x: usize, y: usize) -> Option<&C> {
        assert!(check_not_border(x, y, self.width, self.height));
        let(x,y) = coord_inner_to_outer(x, y);
        let i = coord_to_index(x, y, self.outer_width);
        self.cells.get(i)
    }
    /// Get a mutable borrow of the cell at the given coordinate
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut C> {
        assert!(check_not_border(x, y, self.width, self.height));
        let(x,y) = coord_inner_to_outer(x, y);
        let i = coord_to_index(x, y, self.outer_width);
        self.cells.get_mut(i)
    }
    /// Set the cell at the given coordinate to the provided cell
    pub fn set(&mut self, x: usize, y: usize, cell: C) {
        assert!(check_not_border(x, y, self.width, self.height));
        let(x,y) = coord_inner_to_outer(x, y);
        let i = coord_to_index(x, y, self.outer_width);
        self.cells[i] = cell;
    }
    /// Get a tuple containing the dimension of the GameBoard as (width, height)
    pub fn dim(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    /// Swap two GameBoards with each other. Useful to update one copy while displaying the other.
    pub fn swap(&mut self, other: &mut Self) {
        assert_eq!(self.dim(), other.dim(), "GameBoards cannot swap because of unequal dimension!");
        std::mem::swap(&mut self.cells, &mut other.cells);
    }
}
impl<'a, C: RandomCell> GameBoard<C> {
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
                cells.push(C::new_rand());
            }
        }
        Self {
            outer_width,
            
            width,
            height,
            cells,
            neighbor_lookup: build_neighbor_lookup(outer_width),
        }
    }
}

/// Private impl
impl<'a, C: Cell> GameBoard<C> {
    #[allow(unused)]
    fn index_outer_to_inner(&self, i: usize) -> usize {
        let i = i as isize;
        let i = i
            - self.outer_width as isize // Subtract upper border
            - 2*(i / self.outer_width as isize) // Subtract right border cell of row above and left border cell of current row up to current one
            + 1; // Correct previous statement for the right border cell of the upper row, which was subtracted as a whole 
        i as usize
    }
    fn index_inner_to_outer(&self, i: usize) -> usize {
        let i = i as isize;
        let i = i 
        + self.outer_width as isize // Add upper border row
        + 2*(i / self.width as isize) // Add right border cell of row above and left border cell of current row up to current one
        + 1; // Correct previous statement for the right border cell of the upper row, which is not included in the previous calculation
        i as usize
    }

    /// Produces a NeighborhoodIterator for the cell at coordinates (x,y)
    #[allow(unused)]
    fn iter_neighbors_outer(&self, x: usize, y: usize) -> NeighborhoodIterator<C> {
        NeighborhoodIterator::new(
            self,
            coord_to_index(x, y, self.outer_width) as isize,
            self.neighbor_lookup.iter(),
        )
    }

    /// Argument has to be a valid index into the inner part of the board
    fn iter_neighbors_index_outer(&self, i: usize) -> NeighborhoodIterator<C> {
        NeighborhoodIterator::new(self, i as isize, self.neighbor_lookup.iter())
    }
    fn get_index_outer(&self, i: usize) -> Option<&C> {
        self.cells.get(i)
    }
    unsafe fn get_unchecked_index_outer(&self, i: usize) -> &C {
        self.cells.get_unchecked(i)
    }
    unsafe fn get_unchecked_mut_index_outer(&mut self, i: usize) -> &mut C {
        self.cells.get_unchecked_mut(i)
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
fn coord_inner_to_outer(x: usize, y: usize) -> (usize, usize) {
    (x + 1, y + 1)
}
#[allow(unused)]
fn coord_outer_to_inner(x: usize, y: usize) -> (usize, usize) {
    (x - 1, y - 1)
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::cell::mock::*;
    #[test]
fn test_coord_roundtrip() {
    let (width, height) = (5, 5);
    for x_inner in 0..width {
        for y_inner in 0..height {
            let (x_outer, y_outer) = coord_inner_to_outer(x_inner, y_inner);
            let (x_back, y_back) = coord_outer_to_inner(x_outer, y_outer);
            assert!(x_back == x_inner && y_back == y_inner);
        }
    }
}
#[test]
fn test_index_roundtrip() {
    let (width, height) = (5, 5);
    let board = GameBoard::new(width, height, CellConway::Dead);
    for i_inner in 0..width * height {
        let i_outer = board.index_inner_to_outer(i_inner);
        assert!(i_inner == board.index_outer_to_inner(i_outer));
    }
}
}