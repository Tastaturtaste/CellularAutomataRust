
use rand::{Rng, distributions::{Distribution, Standard}};

pub trait Cell: Clone + Copy + PartialEq{
    fn new_rand() -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellConway{
    Dead,
    Alive,
}
impl Cell for CellConway {
    fn new_rand() -> CellConway {
        rand::random()
    }
}
impl Distribution<CellConway> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CellConway {
        match rng.gen_range(0..=1) {
            0 => CellConway::Dead,
            _ => CellConway::Alive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_send<T: Send>(_: T){}
    #[test]
    fn test_send(){
        is_send(CellConway::Alive);
        is_send(CellConway::Dead);
        let cell = CellConway::Alive;
        is_send(&cell);
    }
}