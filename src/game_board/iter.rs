use super::*;
use crate::cell::*;

pub struct GameBoardIterator<'a, C: Cell> {
    board: &'a GameBoard<C>,
    // iter: I,
    offset: usize,
    end: usize,
}
impl<'a, C: Cell> Iterator for GameBoardIterator<'a, C> {
    type Item = &'a C;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.end {
            return None;
        }

        // if !self.board.check_index(self.offset) {
        //     self.offset += 2;
        //     debug_assert!(self.offset < self.end, "GameBoardIterator in invalid state!");
        // }
        self.offset += 2 * ((self.offset % self.board.inner_width) == 0) as usize;
        // Bound checks are done above and should not be repeated
        let result = unsafe { self.board.get_unchecked_index_inner(self.offset) };
        self.offset += 1;
        Some(result)
    }
    // fn next(&mut self) -> Option<Self::Item> {
    //     self.iter.next()
    //     // self.range.into_iter().filter(|i| self.board.check_index(i)).map(|i| self.board.cells.get_unchecked(i))
    // }
}
impl<'a, C: Cell> GameBoardIterator<'a, C> {
    pub fn new(board: &'a GameBoard<C>, offset: usize, end: usize) -> GameBoardIterator<'a, C> {
        GameBoardIterator { board, offset, end }
    }
}

pub struct GameBoardMutIterator<'a, C: Cell> {
    board: &'a mut GameBoard<C>,
    // iter: I,
    offset: usize,
    end: usize,
}
impl<'a, C: Cell> Iterator for GameBoardMutIterator<'a, C> {
    type Item = &'a mut C;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.end {
            return None;
        }
        self.offset += 2 * ((self.offset % self.board.inner_width) == 0) as usize;
        // Bound checks are done above and should not be repeated
        // Use unsafe pointer to convince the compiler that no two calls to next result in a reference to the same object
        let ptr = unsafe { self.board.get_unchecked_mut_index_inner(self.offset) as *mut C };
        self.offset += 1;
        unsafe { ptr.as_mut() }
    }

    // fn next(&mut self) -> Option<Self::Item> {
    //     self.iter.next()
    //     // self.range.into_iter().filter(|i| self.board.check_index(i)).map(|i| self.board.cells.get_unchecked_mut_index(i))
    // }
}
impl<'a, C: Cell> GameBoardMutIterator<'a, C> {
    pub fn new(
        board: &'a mut GameBoard<C>,
        offset: usize,
        end: usize,
    ) -> GameBoardMutIterator<'a, C> {
        GameBoardMutIterator { board, offset, end }
    }
}

// impl<C: Cell + Send> IntoParallelIterator for GameBoard<C> {
//     type Item = C;
//     type Iter = rayon::vec::IntoIter<Self::Item>;
//     fn into_par_iter(self) -> Self::Iter {
//         self.cells.into_par_iter()
//     }
// }
// impl<'a, C: Cell + Sync> IntoParallelIterator for &'a GameBoard<C> {
//     type Item = &'a C;
//     type Iter = rayon::slice::Iter<'a, C>;
//     fn into_par_iter(self) -> Self::Iter {
//         self.cells.par_iter()
//     }
// }
// impl<'a, C: Cell + Sync + Send> IntoParallelIterator for &'a mut GameBoard<C> {
//     type Item = &'a mut C;
//     type Iter = rayon::slice::IterMut<'a, C>;
//     fn into_par_iter(self) -> Self::Iter {
//         self.cells.par_iter_mut()
//     }
// }

// impl< C: Cell> IntoIterator for GameBoard<C> {
//     type Item = C;
//     type IntoIter = GameBoardIntoIterator<C>; //std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         let iter = (self.inner_width + 1..self.cells.len() - self.inner_width).into_iter().filter(move |&i| self.check_index(i)).map(move |i| unsafe {self.cells.get_unchecked(i)});
//         GameBoardIntoIterator{
//             board: self,
//             iter,
//         }
//     }
// }
impl<'a, C: Cell> IntoIterator for &'a GameBoard<C> {
    type Item = &'a C;
    type IntoIter = GameBoardIterator<'a, C>; //std::slice::Iter<'a, C>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, C: Cell> IntoIterator for &'a mut GameBoard<C> {
    type Item = &'a mut C;
    type IntoIter = GameBoardMutIterator<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Debug)]
pub struct NeighborhoodIterator<'a, C: Cell> {
    board: &'a GameBoard<C>,
    center: isize, // Store as signed integer to make arithmatic easier
    offset_iter: std::slice::Iter<'a, isize>,
}
impl<'a, C: Cell> Iterator for NeighborhoodIterator<'a, C> {
    type Item = &'a C;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.center + self.offset_iter.next()?;
        debug_assert!(self.board.cells.len() > i as usize && 0 <= i);
        unsafe { Some(self.board.get_unchecked_index_inner(i as usize)) }
    }
}
impl<'a, C: Cell> NeighborhoodIterator<'a, C> {
    pub fn new(
        board: &'a GameBoard<C>,
        center: isize,
        offset_iter: std::slice::Iter<'a, isize>,
    ) -> NeighborhoodIterator<'a, C> {
        NeighborhoodIterator {
            board,
            center,
            offset_iter,
        }
    }
}

pub struct LocalGroupIterator<'a, C: Cell> {
    board: &'a GameBoard<C>,
    center: usize,
}

impl<'a, C: Cell> Iterator for LocalGroupIterator<'a, C> {
    type Item = (&'a C, NeighborhoodIterator<'a, C>);
    fn next(&mut self) -> Option<Self::Item> {
        self.center += 2 * ((self.center % self.board.inner_width) == 0) as usize;
        // Bound checks are done above and should not be repeated
        let result = Some((
            unsafe { self.board.get_unchecked_index_inner(self.center as usize) },
            // self.board.get_index_inner(self.center as usize)?,
            self.board.iter_neighbors_index_inner(self.center),
        ));
        self.center += 1;
        result
    }
}
impl<'a, C: Cell> LocalGroupIterator<'a, C> {
    pub fn new(board: &'a GameBoard<C>, center: usize) -> LocalGroupIterator<'a, C> {
        LocalGroupIterator { board, center }
    }
}
