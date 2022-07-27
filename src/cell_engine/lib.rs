pub mod cell;
pub mod default_game_runner;
pub mod default_window;
pub mod game;
pub mod game_board;
pub mod game_rules;
mod globals;
pub mod rgba;
pub mod visuals;

#[macro_export]
macro_rules! dprintln {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::println!($($rest)*)
    }
}
