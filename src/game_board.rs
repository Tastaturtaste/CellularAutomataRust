pub use crate::cell::*;

use core::num;
use std::iter::IntoIterator;


#[derive(Default, Debug, Clone)]
pub struct GameBoard {
    width:u16,
    height:u16,
    cells: Vec<Cell>,
}

impl GameBoard {
    pub fn new(width:u16, height:u16) -> Self {
        let num_nodes = (width*height) as usize;
        Self{
            width,
            height,
            cells: vec![Cell::Dead; num_nodes],
        }
    }
    pub fn new_rand(width:u16, height:u16) -> Self {
        let mut cells = Vec::new();
        let num_elements = width as usize * height as usize;
        cells.reserve(num_elements);
        for _ in 0..(num_elements) {
            cells.push(Cell::new_rand());
        }
        Self::from_slice(&mut cells, width, height).unwrap()
    }
    pub fn from_slice(vec: &mut [Cell], width:u16, height:u16) -> Result<GameBoard,&str> {
        let num_elements = width as usize * height as usize;
        if vec.len() != num_elements {
            Err("Wrong number of elements!")
        } else{
            Ok(GameBoard{width, height, cells: vec.to_vec()})
        }
    }
    fn index(&self, x: u16, y: u16) -> usize { x as usize + (self.width as usize * y as usize) }
    pub fn check_index(&self, x: i32,y: i32) -> bool{ x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32}

    pub fn iter(&self) -> std::slice::Iter<'_,Cell> {
        self.cells.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_,Cell> {
        self.cells.iter_mut()
    }

    pub fn iter_neighbors(&self, x: u16, y: u16) -> NeighborhoodIterator {
        NeighborhoodIterator::new(&self, (x, y))
    }

    pub fn get(self: &Self, x:u16, y: u16) -> Option<Cell> {
        self.cells.get(self.index(x, y)).cloned()
    }
    pub fn get_mut(self: &mut Self, x: u16, y: u16) -> Option<&mut Cell>{
        let index = self.index(x, y);
        self.cells.get_mut(index)
    }
    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        let index = self.index(x,y);
        self.cells[index] = cell;
    }
    #[inline]
    pub fn dim(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    pub fn to_vec(self) -> Vec<Cell> {
        self.cells
    }
    pub fn from<I>(iter: I, width:u16, height:u16) -> Result<GameBoard,&'static str>
    where I: std::iter::ExactSizeIterator<Item=Cell>
    {
        if iter.len() != (width*height) as usize {
            Err("Wrong number of elements!")
        } else{
            Ok(GameBoard{width,height,cells: iter.collect::<Vec<Cell>>()})
        }
    }
    pub fn swap(&mut self, other: &mut Self) {
        // if !(self.width == other.width) || !(self.height == other.height) {
        //     panic!("GameBoards cannot swap because of unequal dimension!");
        // }
        if !(self.dim() == other.dim()) {
            panic!("GameBoards cannot swap because of unequal dimension!");
        }
        std::mem::swap(&mut self.cells, &mut other.cells);
    }
}


impl<'a> IntoIterator for GameBoard{
    type Item = Cell;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        return self.cells.into_iter();
    }
}
impl<'a> IntoIterator for &'a GameBoard{
    type Item = &'a Cell;
    type IntoIter = std::slice::Iter<'a,Cell>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a> IntoIterator for &'a mut GameBoard{
    type Item = &'a mut Cell;
    type IntoIter = std::slice::IterMut<'a,Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Debug)]
pub struct NeighborhoodIterator<'a> {
    board: &'a GameBoard,
    central: (i32,i32), // Store as signed integer to make arithmatic easier
    counter: u8,
}
impl<'a> NeighborhoodIterator<'a> {
    const OFFSETS : [(i8,i8);8] = [
        (-1,-1),
        (0,-1),
        (1,-1),
        (-1,0),
        (1,0),
        (-1,1),
        (0,1),
        (1,1)
    ];
}

impl<'a> NeighborhoodIterator<'a> {
    fn new(board: &'a GameBoard, central: (u16,u16)) -> Self {
        NeighborhoodIterator{board, central: (central.0 as i32, central.1 as i32), counter: 0}
    }
    #[inline]
    const fn is_viable(x: i32, y:i32, width: i32, height: i32) -> bool {
        (x >= 0) && (x < width) && (y >= 0) && (y < height)
    }

    #[doc = "Get the next counter which results in a viable offset"]
    fn get_next_viable(&self, counter: u8) -> Option<u8> {
        for counter in (counter+1).into()..Self::OFFSETS.len(){
            let (new_dx,new_dy) = Self::OFFSETS[counter as usize];
            let tmp_x: i32 = self.central.0 as i32 + new_dx as i32;
            let tmp_y: i32 = self.central.1 as i32 + new_dy as i32;
            if Self::is_viable(tmp_x,tmp_y,self.board.width as i32, self.board.height as i32) {
                return Some(counter as u8);
            }
        }
        None
    }
}

impl<'a> Iterator for NeighborhoodIterator<'a>{
    type Item = Cell;
    fn next(self: &mut NeighborhoodIterator<'a>) -> Option<Self::Item> {
        if self.counter as usize == Self::OFFSETS.len(){
            return None;
        }
        let offset = Self::OFFSETS[self.counter as usize];
        let (x,y) = (
            self.central.0 as i32 + offset.0 as i32, 
            self.central.1 as i32 + offset.1 as i32,
        );
        if !Self::is_viable(x, y, self.board.width as i32, self.board.height as i32){
            self.counter = self.get_next_viable(self.counter)?;
        }
        let (new_dx,new_dy) = Self::OFFSETS[self.counter as usize];
        let x = self.central.0 + new_dx as i32;
        let y = self.central.1 + new_dy as i32;
        let value = self.board.get(x as u16 , y as u16);
        self.counter += 1;
        value
    }
}

#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn iterate_neighborhood(){
        let mut board = GameBoard::new(10,10);
        board.set(0,0,Cell::Alive);
        for n in board.iter_neighbors(0, 0){
            println!("{:?}",n);
        }
    }
    #[test]
    fn neighbor_count_corner(){
        let mut board = GameBoard::new(10,10);
        for c in &mut board{
            *c = Cell::Alive;
        }
        let sum: i32 = board.iter_neighbors(0,0).map(|c| match c {Cell::Dead => 0, Cell::Alive => 1}).sum();
        assert_eq!(sum,3);
    }
    #[test]
    fn iterate_board(){
        let board = GameBoard::new(10,10);
        for n in board.iter(){
            println!("{:?}",n);
        }
    }
    #[test]
    fn iterate_board_and_set_alive(){
        let mut board = GameBoard::new(10,10);
        for n in board.iter_mut(){
            *n = Cell::Alive;
        }
        for n in board.iter() {
            println!("{:?}",n);
        }
    }
}