mod cell;
mod globals;
mod wireworld;
use cell::WireCell;
use cell_engine::default_game_runner::GameRunner;
use wireworld::*;

fn main() {
    let cell_size = 16;
    let width = 2550 / cell_size;
    let height = 1440 / cell_size;
    let border_cell = cell::WireCell::Off;
    let game = wireworld::WireGame::new(width, height, border_cell);
    let decay_decider = |_c: &WireCell| false;
    let game_runner = GameRunner::new(decay_decider);
    game_runner.run(game, "Wireworld");
}
