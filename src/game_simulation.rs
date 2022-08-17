pub mod simulate_game {
    use crate::matrix::matrix::SmallMatrix;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    #[derive(Clone, Default)]
    pub struct Cell {
        curr_r: u8,
        curr_g: u8,
        curr_b: u8,

        next_r: u8,
        next_g: u8,
        next_b: u8
    }

    impl Cell {
        fn new(r:u8, g:u8, b:u8) -> Self {
            Self {curr_r:r, curr_g:g, curr_b:b,
                next_r: 0, next_g: 0, next_b: 0 }
        }

        fn advance_gen(& mut self) {
            self.curr_r = self.next_r;
            self.curr_g = self.next_g;
            self.curr_b = self.next_b;
        }
        fn is_alive(&self) -> bool {
            self.curr_r > 0 || self.curr_g > 0 || self.curr_b > 0
        }

        fn set_dead(& mut self) {
            self.next_r = 0;
            self.next_g = 0;
            self.next_b = 0;
        }
    }

    pub fn initialize_game(n: usize, m: usize, seed: u64) -> SmallMatrix<Cell>{
        let mut grid: SmallMatrix<Cell> = match SmallMatrix::new(n, m) {
            Ok(matrix) => matrix,
            Err(_) => panic!("Unachievable size of matrix")
        };

        populate_grid(& mut grid, seed);
        return grid
    }

    fn populate_grid(grid: & mut SmallMatrix<Cell>, seed:u64) {
        let (num_rows, num_columns) = grid.get_size();
        let mut randomizer = ChaCha8Rng::seed_from_u64(seed);
        for x in 0..num_rows {
            for y in 0..num_columns {
                // Randomly set some cells to be alive.
                if randomizer.gen_bool(0.5) {
                    let cell = grid.get_mut_val_at(x, y).unwrap();
                    cell.curr_r = randomizer.gen_range(0..255);
                    cell.curr_g = randomizer.gen_range(0..255);
                    cell.curr_b = randomizer.gen_range(0..255);
                }
            }
        }
    }

    pub fn next_generation(grid: & mut SmallMatrix<Cell>) {
        let (num_rows, num_columns) = grid.get_size();
        for x in 0..num_rows {
            for y in 0..num_columns {
                let (num_living, avg_r, avg_g, avg_b) = get_neighbor_data(&grid, x, y);
                let mut cell = grid.get_mut_val_at(x, y).unwrap();
                if num_living < 2 {
                    cell.set_dead();
                }
                else if num_living > 3 {
                    cell.set_dead();
                }
                else if num_living == 3 {
                    if cell.is_alive() {
                        cell.next_r = 255 - avg_r;
                        cell.next_g = 255 - avg_g;
                        cell.next_b = 255 - avg_b;
                    }
                    else {
                        cell.next_r = avg_r;
                        cell.next_g = avg_g;
                        cell.next_b = avg_b;
                    }
                }
            }
        }
        for x in 0..num_rows {
            for y in 0..num_columns {
                let cell = grid.get_mut_val_at(x, y).unwrap();
                cell.advance_gen();
            }
        }
    }

    pub fn as_vector(grid: &SmallMatrix<Cell>) -> Vec<u32> {
        let (num_rows, num_columns) = grid.get_size();
        let mut vector: Vec<u32> = Vec::with_capacity(num_columns * num_rows);

        for x in 0..num_rows {
            for y in 0..num_columns {
                let cell = grid.get_val_at(x, y).unwrap();
                let mut color: u32 = (cell.curr_r as u32) << 16;
                color += (cell.curr_g as u32) << 8;
                color += cell.curr_b as u32;
                vector.push(color);
            }
        }
        return vector;
    }

    fn get_neighbor_data(grid: &SmallMatrix<Cell>, x: usize, y: usize) -> (u8, u8, u8, u8) {
        let mut num_living_neighbors: u8 = 0;
        let mut avg_r: u16 = 0;
        let mut avg_g: u16 = 0;
        let mut avg_b: u16 = 0;

        let (x_bound, y_bound) = grid.get_size();

        let lowest_x= if  x == 0 {0} else {x - 1};
        let highest_x= if x + 1 == x_bound {x} else {x + 1};
        let lowest_y = if y == 0 {0} else {y - 1};
        let highest_y= if y + 1 == y_bound {y} else {y + 1};

        for x_offset in lowest_x..=highest_x {
            for y_offset in lowest_y..=highest_y {
                if x == 0 && y == 0 {
                    continue
                }
                // Use unwrap here because if we access something out of bounds the program is not
                // working correctly.
                let neighbor = grid.get_val_at(x_offset, y_offset).unwrap();
                if neighbor.is_alive() {
                    num_living_neighbors += 1;
                    avg_r += neighbor.next_r as u16;
                    avg_g += neighbor.next_g as u16;
                    avg_b += neighbor.next_b as u16;

                }
            }
        }
        if num_living_neighbors > 0 {
            avg_r /= num_living_neighbors as u16;
            avg_g /= num_living_neighbors as u16;
            avg_b /= num_living_neighbors as u16;
        }
        (num_living_neighbors, avg_r as u8, avg_g as u8, avg_b as u8)
    }
}