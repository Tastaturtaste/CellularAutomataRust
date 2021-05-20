use super::*;
use crate::cell::*;

pub fn build_neighbor_lookup(inner_width: usize) -> [isize; 8] {
    [
        -(inner_width as isize) - 1,
        -(inner_width as isize),
        -(inner_width as isize) + 1,
        -1,
        1,
        (inner_width as isize) - 1,
        (inner_width as isize),
        (inner_width as isize) + 1,
    ]
}

mod inner {
    use super::*;

    impl<'a, C> GameBoard<C>
    where
        C: Cell,
    {
        // fn from_slice(vec: &mut [C], inner_width:u16, inner_height:u16, void_cell: C) -> Result<Self,&str> {
        //     let num_elements = width as usize * height as usize;
        //     if vec.len() != num_elements {
        //         Err("Wrong number of elements!")
        //     } else{
        //         Ok(Self{width, height, cells: vec.to_vec(), void_cell})
        //     }
        // }
        pub fn coord_to_index_inner(&self, x: usize, y: usize) -> usize {
            let i = x + (self.inner_width * y);
            debug_assert!(i < self.cells.len());
            i
        }
        pub fn index_to_coord_inner(&self, i: usize) -> (usize, usize) {
            debug_assert!(i < self.cells.len());
            (
                (i % self.inner_width as usize),
                (i / self.inner_width as usize),
            )
        }
        #[inline]
        pub fn check_index_inner(&self, i: usize) -> bool {
            let wrapx = i % self.inner_width as usize;
            let wrapy = i / self.inner_width as usize;
            self.check_coord_inner(wrapx, wrapy)
        }
        #[inline]
        pub fn check_coord_inner(&self, x: usize, y: usize) -> bool {
            x > 0 && x < (self.inner_width) && y > 0 && y < (self.inner_height)
        }
        #[inline]
        pub fn coord_inner_to_public(&self, x: usize, y: usize) -> (usize, usize) {
            debug_assert!(
                x >= 1 && x < (self.inner_width - 1) && y >= 1 && y < (self.inner_height - 1)
            );
            (x - 1, y - 1)
        }
        #[inline]
        pub fn coord_public_to_inner(&self, x: usize, y: usize) -> (usize, usize) {
            debug_assert!(x < (self.width) && y < (self.height));
            (x + 1, y + 1)
        }
        #[inline]
        pub fn index_inner_to_public(&self, i: usize) -> usize {
            debug_assert!(
                i > (self.inner_width + 1) && i < (self.inner_width + 1 + self.width * self.height)
            );
            let i = i as isize;
            let i = i - self.inner_width as isize - (i / self.inner_width as isize) * 2 + 1;
            i as usize
        }
        #[inline]
        pub fn index_public_to_inner(&self, i: usize) -> usize {
            debug_assert!(i < (self.width * self.height));
            let i = i as isize;
            let i = i + self.inner_width as isize + (i / self.width as isize) * 2 + 1;
            i as usize
        }

        pub fn iter_neighbors_inner(&self, x: usize, y: usize) -> NeighborhoodIterator<C> {
            // Allow only indexing into the inner parts of the board
            debug_assert!(self.check_coord_inner(x, y));
            NeighborhoodIterator::new(
                self,
                self.coord_to_index_inner(x, y) as isize,
                self.neighbor_lookup.iter(),
            )
        }
        pub fn iter_neighbors_index_inner(&self, i: usize) -> NeighborhoodIterator<C> {
            // Allow only indexing into the inner parts of the board
            debug_assert!(self.check_index_inner(i));
            NeighborhoodIterator::new(self, i as isize, self.neighbor_lookup.iter())
        }
        pub unsafe fn get_unchecked_inner(self: &Self, x: usize, y: usize) -> &C {
            self.cells.get_unchecked(self.coord_to_index_inner(x, y))
        }
        pub fn get_index_inner(self: &Self, i: usize) -> Option<&C> {
            self.cells.get(i)
        }
        pub unsafe fn get_unchecked_index_inner(self: &Self, i: usize) -> &C {
            self.cells.get_unchecked(i)
        }
        pub unsafe fn get_unchecked_mut_index_inner(self: &mut Self, i: usize) -> &mut C {
            self.cells.get_unchecked_mut(i)
        }
        pub fn get_or<'b, 'c>(self: &'a Self, x: usize, y: usize, backup_cell: &'b C) -> &'c C
        where
            'a: 'c,
            'b: 'c,
        {
            match self.cells.get(self.coord_to_index_inner(x, y)) {
                Some(cell) => cell,
                None => backup_cell,
            }
        }
        pub fn to_vec(self) -> Vec<C> {
            self.cells
        }
    }
}
