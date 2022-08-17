pub mod simulate_game {
    use std::u32::MAX;
    use crate::matrix::matrix::SmallMatrix;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;


    pub fn run_game(n: usize, m: usize, seed: u64) {
        let mut grid: SmallMatrix<u32> = match SmallMatrix::new(n, m) {
            Ok(matrix) => matrix,
            Err(_) => panic!("Unachievable size of matrix")
        };

        populate_grid(& mut grid, seed);
    }

    pub fn initialize_game(n: usize, m: usize, seed: u64) -> SmallMatrix<u32>{
        let mut grid: SmallMatrix<u32> = match SmallMatrix::new(n, m) {
            Ok(matrix) => matrix,
            Err(_) => panic!("Unachievable size of matrix")
        };

        populate_grid(& mut grid, seed);
        return grid
    }

    fn populate_grid(grid: & mut SmallMatrix<u32>, seed:u64) {
        let (num_rows, num_columns) = grid.get_size();
        let mut randomizer = ChaCha8Rng::seed_from_u64(seed);
        for x in 0..num_rows {
            for y in 0..num_columns {
                let new_cell: u32 = if randomizer.gen_bool(0.01) {
                    MAX
                } else {
                    0
                };
                match grid.set_val_at(x, y, new_cell) {
                    Ok(_) => (),
                    Err(_) => panic!("Grid accessed out of bounds")
                }
            }
        }
    }

    pub fn next_generation(grid: &mut SmallMatrix<u32>) {
        let (num_rows, num_columns) = grid.get_size();
        for x in 0..num_rows {
            for y in 0..num_columns {
                let num_living = get_neighbor_data(&grid, x, y);
                // less than 2 neighbors dies.
                if num_living < 2 || num_living > 3 {
                    grid.set_val_at(x, y, 0).expect("Out of bounds access");
                }
                else if num_living == 3 {
                    grid.set_val_at(x, y, MAX).expect("Out of bounds access");
                }
            }
        }
    }

    pub fn as_vector (grid: &SmallMatrix<u32>) -> &Vec<u32> {
        &grid.data
    }

    fn get_neighbor_data(grid: &SmallMatrix<u32>, x: usize, y: usize) -> (u8) {
        let mut num_living_neighbors = 0;

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
                if neighbor > &0 {
                    num_living_neighbors += 1;
                }
            }
        }
        num_living_neighbors
    }
}