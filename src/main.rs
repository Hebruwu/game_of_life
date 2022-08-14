extern crate core;

mod game_simulation;
mod matrix;

fn main() {
    game_simulation::simulate_game::game_start(100, 100);
}
