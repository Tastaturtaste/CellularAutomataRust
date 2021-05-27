mod cell;
mod game_of_life;
use cell::CellConway;
use cell_engine::default_game_runner::GameRunner;
use cell_engine::game::traits::*;
use game_of_life::ConwaysGame;

fn main() {
    let cell_size = 8;
    let width = 2550 / cell_size;
    let height = 1440 / cell_size;
    let border_cell = cell::CellConway::Dead;
    let game = ConwaysGame::new_rand(width, height, border_cell);
    let overwrite_decaying = |c: &CellConway| match *c {
        CellConway::Alive => true,
        CellConway::Dead => false,
    };
    let game_runner = GameRunner::<ConwaysGame>::new(overwrite_decaying);
    game_runner.run(game, "Game of Life");
}
