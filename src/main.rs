extern crate core;

mod game_simulation;
mod matrix;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use game_simulation::simulate_game;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut grid = simulate_game::initialize_game(WIDTH, HEIGHT, 15);
    let mut buffer:Vec<u32>;

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        simulate_game::next_generation(& mut grid);
        buffer = simulate_game::as_vector(&grid);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}