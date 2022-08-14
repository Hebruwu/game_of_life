
pub mod simulate_game {
    use crate::matrix::matrix::SmallMatrix;

    struct Cell {
        alive: bool,
        r: u8,
        g: u8,
        b: u8
    }

    pub fn game_start(n: usize, m: usize) {
        let grid: SmallMatrix<u8> = match SmallMatrix::new(n, m) {
            Ok(matrix) => matrix,
            Err(e) => panic!("Unachievable size of matrix")
        };
    }
}