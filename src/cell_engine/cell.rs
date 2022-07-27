use crate::rgba::RGBA;

pub trait Cell: Clone + Copy + PartialEq {
    fn to_rgba(&self) -> RGBA;
    fn next(&self) -> Self;
}

pub trait RandomCell: Cell {
    fn new_rand() -> Self;
}

pub(crate) mod mock {
    pub use super::*;
    use rand::{
        distributions::{Distribution, Standard},
        Rng,
    };
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum CellConway {
        Dead,
        Alive,
    }

    impl Distribution<CellConway> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CellConway {
            match rng.gen_range(0..=1) {
                0 => CellConway::Dead,
                _ => CellConway::Alive,
            }
        }
    }

    impl RandomCell for CellConway {
        fn new_rand() -> Self {
            rand::random()
        }
    }

    impl Cell for CellConway {
        fn to_rgba(&self) -> RGBA {
            match *self {
                Self::Dead => RGBA::black(),
                Self::Alive => RGBA::white(),
            }
        }
        fn next(&self) -> Self {
            match *self {
                Self::Dead => Self::Alive,
                Self::Alive => Self::Dead,
            }
        }
    }
}
