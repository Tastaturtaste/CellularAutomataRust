pub use crate::cell::*;
use rayon::prelude::*;

use std::{iter::IntoIterator};


#[derive(Default, Debug, Clone)]
pub struct GameBoard<C: Cell>
{
    width:u16,
    height:u16,
    cells: Vec<C>,
    void_cell: C,
}

impl<'a, C> GameBoard<C>
where
    C: Cell,
{
    pub fn new(width:u16, height:u16, void_cell: C) -> Self {
        // void_cell represents all cells outside of the game board
        let num_nodes = (width*height) as usize;
        Self{
            width,
            height,
            cells: vec![void_cell.clone(); num_nodes],
            void_cell,
        }
    }
    pub fn new_rand(width:u16, height:u16, void_cell: C) -> Self {
        let mut cells = Vec::new();
        let num_elements = width as usize * height as usize;
        cells.reserve(num_elements);
        for _ in 0..(num_elements) {
            cells.push(Cell::new_rand());
        }
        Self::from_slice(&mut cells, width, height, void_cell).unwrap()
    }
    pub fn from_slice(vec: &mut [C], width:u16, height:u16, void_cell: C) -> Result<Self,&str> {
        let num_elements = width as usize * height as usize;
        if vec.len() != num_elements {
            Err("Wrong number of elements!")
        } else{
            Ok(Self{width, height, cells: vec.to_vec(), void_cell})
        }
    }
    fn index(&self, x: u16, y: u16) -> usize { x as usize + (self.width as usize * y as usize) }
    pub fn index_to_coord(&self, i: usize) -> (u16,u16) { 
        (
            (i % self.width as usize) as u16, 
            (i / self.width as usize) as u16
        )
    }
    pub fn check_index(&self, x: i32,y: i32) -> bool{ x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32}

    pub fn iter(&self) -> std::slice::Iter<'_,C> {
        self.cells.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_,C> {
        self.cells.iter_mut()
    }
    pub fn iter_neighbors(&self, x: u16, y: u16) -> NeighborhoodIterator<C> {
        NeighborhoodIterator::new(&self, (x, y))
    }
    pub fn iter_local_groups(&self) -> LocalGroupIterator<C> {
        LocalGroupIterator{board:&self, center_enumeration: self.cells.iter().enumerate() }
    }
    pub fn get(self: &Self, x:u16, y: u16) -> Option<&C> {
        self.cells.get(self.index(x, y))
    }
    pub fn get_or<'b,'c>(self: &'a Self, x:u16, y: u16, backup_cell: &'b C) -> &'c C
    where
    'a: 'c,
    'b: 'c,
    {
        match self.cells.get(self.index(x as u16, y as u16)) {
            Some(cell) => cell,
            None => backup_cell,
        }
    }
    pub fn get_or_void(&self, x:u16, y:u16) -> &C {
        self.get_or(x,y,&self.void_cell)
    }
    pub fn get_mut(self: &mut Self, x: u16, y: u16) -> Option<&mut C>{
        let index = self.index(x, y);
        self.cells.get_mut(index)
    }
    pub fn set(&mut self, x: u16, y: u16, cell: C) {
        let index = self.index(x,y);
        self.cells[index] = cell;
    }
    #[inline]
    pub fn dim(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    pub fn to_vec(self) -> Vec<C> {
        self.cells
    }
    pub fn swap(&mut self, other: &mut Self) {
        if !(self.dim() == other.dim()) {
            panic!("GameBoards cannot swap because of unequal dimension!");
        }
        std::mem::swap(&mut self.cells, &mut other.cells);
    }
}

impl<C: Cell + Send> IntoParallelIterator for GameBoard<C>{
    type Item = C;
    type Iter = rayon::vec::IntoIter<Self::Item>;
    fn into_par_iter(self) -> Self::Iter {
        self.cells.into_par_iter()
    }
}
impl<'a,C: Cell + Sync> IntoParallelIterator for &'a GameBoard<C>{
    type Item = &'a C;
    type Iter = rayon::slice::Iter<'a,C>;
    fn into_par_iter(self) -> Self::Iter {
        self.cells.par_iter()
    }
}
impl<'a,C: Cell + Sync + Send> IntoParallelIterator for &'a mut GameBoard<C>{
    type Item = &'a mut C;
    type Iter = rayon::slice::IterMut<'a,C>;
    fn into_par_iter(self) -> Self::Iter {
        self.cells.par_iter_mut()
    }
}

impl<'a, C: Cell> IntoIterator for GameBoard<C>{
    type Item = C;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        return self.cells.into_iter();
    }
}
impl<'a, C: Cell> IntoIterator for &'a GameBoard<C>{
    type Item = &'a C;
    type IntoIter = std::slice::Iter<'a,C>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, C: Cell> IntoIterator for &'a mut GameBoard<C>{
    type Item = &'a mut C;
    type IntoIter = std::slice::IterMut<'a,C>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Debug)]
pub struct NeighborhoodIterator<'a, C: Cell> {
    board: &'a GameBoard<C>,
    central: (i32,i32), // Store as signed integer to make arithmatic easier
    counter: usize,
}
const OFFSETS : [(i32,i32);8] = [
    (-1,-1),
    (0,-1),
    (1,-1),
    (-1,0),
    (1,0),
    (-1,1),
    (0,1),
    (1,1)
];
impl<'a, C: Cell> NeighborhoodIterator<'a, C> {
    fn new(board: &'a GameBoard<C>, central: (u16,u16)) -> Self {
        NeighborhoodIterator{board, central: (central.0 as i32, central.1 as i32), counter:0}
    }
}

type AddOut<T> =  <T as std::ops::Add>::Output;
fn add_tuples<T>(tup1: &(T,T), tup2: &(T,T)) -> (AddOut<T>, AddOut<T>) 
where T: std::ops::Add + Copy,
{
    (
        tup1.0 + tup2.0,
        tup1.1 + tup2.1,
    )
}

impl<'a, C: Cell> Iterator for NeighborhoodIterator<'a, C>{
    type Item = &'a C;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < OFFSETS.len() {
            let (x,y) = add_tuples(&self.central, &OFFSETS[self.counter]);
            self.counter += 1;
            match (x,y){
                (x,y) if (x > 0 && y > 0) => Some(self.board.get_or_void(x as u16, y as u16)),
                _ => Some(&self.board.void_cell)
            }
        }
        else{
            None
        }
    }
}

pub struct LocalGroupIterator<'a,C: Cell> {
    board: &'a GameBoard<C>,
    center_enumeration: std::iter::Enumerate<std::slice::Iter<'a,C>>,
}

impl<'a, C: Cell> Iterator for LocalGroupIterator<'a, C> {
    type Item = (&'a C, NeighborhoodIterator<'a, C>);
    fn next(&mut self) -> Option<Self::Item> {
        self.center_enumeration.next()
        .map(|(i,c)| {
            let (x,y) = self.board.index_to_coord(i);
            (
                c, 
                self.board.iter_neighbors(x,y),
            )
        })
    }
}

#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use super::*;
    use CellConway::*;
    #[test]
    fn iterate_neighborhood(){
        let mut board = GameBoard::new(10,10,Dead);
        board.set(0,0,Alive);
        for n in board.iter_neighbors(0, 0){
            println!("{:?}",n);
        }
    }
    #[test]
    fn neighbor_count_corner(){
        let mut board = GameBoard::new(10,10,Dead);
        for c in &mut board{
            *c = Alive;
        }
        let sum: i32 = board.iter_neighbors(0,0).map(|c| match c {Dead => 0, Alive => 1}).sum();
        assert_eq!(sum,3);
    }
    #[test]
    fn iterate_board(){
        let board = GameBoard::new(10,10, Dead);
        for n in board.iter(){
            println!("{:?}",n);
        }
    }
    #[test]
    fn iterate_board_and_set_alive(){
        let mut board = GameBoard::new(10,10, Dead);
        for n in board.iter_mut(){
            *n = Alive;
        }
        for n in board.iter() {
            println!("{:?}",n);
        }
    }
}