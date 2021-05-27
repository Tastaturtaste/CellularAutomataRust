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
    let overwrite_decaying = |c: &WireCell| match *c {
        WireCell::ElectronHead => true,
        WireCell::ElectronTail => true,
        _ => false,
    };
    let game_runner = GameRunner::new(overwrite_decaying);
    game_runner.run(game, "Wireworld");
}
