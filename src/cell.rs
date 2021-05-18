
use rand::{Rng, distributions::{Distribution, Standard}};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell{
    Dead,
    Alive,
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0..=1) {
            0 => Cell::Dead,
            _ => Cell::Alive,
        }
    }
}
impl Cell {
    pub fn new_rand() -> Cell {
        rand::random()
    }
}