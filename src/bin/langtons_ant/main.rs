use cell_engine::{
    cell::{Cell, RandomCell},
    default_game_runner::GameRunner,
    game::{traits::CellGame as CellGameTrait, CellGame},
    game_rules::GameRule,
    rgba::RGBA,
};
use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Ant {
    North,
    West,
    South,
    East,
}
impl Ant {
    fn next(&self) -> Self {
        match *self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum LangtonsCell {
    Black(Option<Ant>),
    White(Option<Ant>),
}
use LangtonsCell::*;
impl Cell for LangtonsCell {
    fn next(&self) -> Self {
        dbg!(*self);
        match *self {
            Black(None) => White(None),
            White(None) => White(Some(Ant::North)),
            White(Some(ant)) if ant != Ant::West => White(Some(ant.next())),
            White(Some(Ant::West)) => Black(Some(Ant::North)),
            Black(Some(ant)) if ant != Ant::West => Black(Some(ant.next())),
            Black(Some(Ant::West)) => Black(None),
            _ => unreachable!(),
        }
    }
    fn to_rgba(&self) -> RGBA {
        match *self {
            White(None) => RGBA::white(),
            White(Some(_)) => blend_rgba_factor(RGBA::white(), RGBA::red(), 0.5),
            Black(None) => RGBA::black(),
            Black(Some(_)) => blend_rgba_factor(RGBA::black(), RGBA::red(), 0.5),
        }
    }
}

impl RandomCell for LangtonsCell {
    fn new_rand() -> Self {
        match rand::thread_rng().gen_range(0..=1) {
            0 => Self::Black(None),
            1 => Self::White(None),
            _ => unreachable!(),
        }
    }
}

fn blend_rgba_factor(rgba_1: RGBA, rgba_2: RGBA, factor: f32) -> RGBA {
    let mut rgba = [0; 4];
    for (i, (&v1, &v2)) in rgba_1.0.iter().zip(rgba_2.0.iter()).enumerate() {
        rgba[i] = (v1 as f32 * factor + v2 as f32 * (1. - factor)) as u8
    }
    RGBA(rgba)
}

#[derive(Default, Clone, Copy)]
struct LangtonsRule {}

impl GameRule for LangtonsRule {
    type Cell = LangtonsCell;

    fn apply(
        cell: &Self::Cell,
        neighbor_iter: cell_engine::game_board::iter::NeighborhoodIterator<Self::Cell>,
    ) -> Self::Cell {
        match *cell {
            White(Some(_)) => return Black(None),
            Black(Some(_)) => return White(None),
            _ => (),
        }
        let mut ant = None;

        // Rule depends on order of iteration of neighbor iterator
        // Assuming iteration starts at top left
        'label: for (i, &neighbor) in neighbor_iter.enumerate() {
            match (i, neighbor) {
                (1, neighbor)
                    if neighbor == White(Some(Ant::East)) || neighbor == Black(Some(Ant::West)) =>
                {
                    ant = Some(Ant::South);
                    break 'label;
                }
                (3, neighbor)
                    if neighbor == White(Some(Ant::South))
                        || neighbor == Black(Some(Ant::North)) =>
                {
                    ant = Some(Ant::West);
                    break 'label;
                }
                (4, neighbor)
                    if neighbor == White(Some(Ant::North))
                        || neighbor == Black(Some(Ant::South)) =>
                {
                    ant = Some(Ant::East);
                    break 'label;
                }
                (6, neighbor)
                    if neighbor == White(Some(Ant::West)) || neighbor == Black(Some(Ant::East)) =>
                {
                    ant = Some(Ant::North);
                    break 'label;
                }
                _ => (),
            }
        }
        match *cell {
            White(_) => White(ant),
            Black(_) => Black(ant),
        }
    }
}

type LangtonsGame = CellGame<LangtonsCell, LangtonsRule>;

fn main() {
    let width = 2550 / 8;
    let height = 1440 / 8;
    let game = LangtonsGame::new(width, height, LangtonsCell::Black(None));
    let overwrite_decaying = |c: &LangtonsCell| matches!(*c, White(Some(_)) | Black(Some(_)));
    let game_runner = GameRunner::new(overwrite_decaying);
    game_runner.run(game, "Langton's Ant");
}
