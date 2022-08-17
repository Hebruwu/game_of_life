pub mod matrix {
    use crate::matrix::errors::{OutOfBoundsError, UnachieveableSize};

    /// Represents a matrix of n rows and m columns.
    pub struct SmallMatrix<T: Default + Clone> {
        rows: usize,
        columns: usize,
        data: Vec<T>,
    }


    impl<T: Default + Clone> SmallMatrix<T> {
        /// Creates a new instance of a matrix and returns it. Note that a matrix cannot have n and
        /// an m such that m * n would cause usize to overflow. Neither n nor m may be 0.  <br/>
        /// * `n`: the number of rows.
        /// * `m`: the number of columns.
        ///
        /// returns: an instance of SmallMatrix.
        pub fn new(n: usize, m: usize) -> Result<Self, UnachieveableSize> {
            let vector_size = m * n;
            if vector_size == 0 || vector_size / n != m {
                return Err(UnachieveableSize)
            }

            Ok(Self {rows: n, columns: m, data:vec![T::default(); n * m]})
        }

        /// Returns a tuple that represents the size of the matrix as (n, m).
        pub fn get_size(&self) -> (usize, usize) {
            return (self.rows, self.columns)
        }

        /// Returns the value located at the (x,y) position in the matrix.
        /// * `x`: the row position.
        /// * `y`: the column position.
        ///
        /// returns: Result<&T, OutOfBoundsError>
        pub fn get_val_at(&self, x: usize, y:usize) -> Result<&T, OutOfBoundsError> {
            if x > self.rows || y > self.columns {
                return Err(OutOfBoundsError)
            }

            let position_in_vector: usize = x + y * self.rows;
            return match self.data.get(position_in_vector) {
                Some(t) => Ok(t),
                None => Err(OutOfBoundsError),
            }
        }

        /// Sets value at a provided (x, y) position in the vector. Returns an empty result if
        /// successful.
        /// * `x`: the row position.
        /// * `y`: the column position.
        /// * `value`: the value to put into the matrix.
        ///
        /// returns: Result<(), OutOfBoundsError>
        pub fn set_val_at(&mut self, x: usize, y: usize, value: T) -> Result<(), OutOfBoundsError> {
            if x > self.rows || y > self.columns {
                return Err(OutOfBoundsError)
            }

            let position_in_vector: usize = x + y * self.rows;
            self.data[position_in_vector] = value;
            return Ok(())
        }

        /// Returns a mutable reference to the value located at the (x,y) position in the matrix.
        /// * `x`: the row position.
        /// * `y`: the column position.
        ///
        /// returns: Result<mut &T, OutOfBoundsError>
        pub fn get_mut_val_at(&mut self, x: usize, y: usize) -> Result<& mut T, OutOfBoundsError> {
            if x > self.rows || y > self.columns {
                return Err(OutOfBoundsError)
            }

            let position_in_vector: usize = x + y * self.rows;
            match self.data.get_mut(position_in_vector) {
                Some(t) => Ok(t),
                None => Err(OutOfBoundsError),
            }
        }
    }
}

mod errors {
    use std::fmt::Formatter;

    pub struct OutOfBoundsError;

    impl std::fmt::Debug for OutOfBoundsError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{ file: {}, line: {} }}", file!(), line!())
        }
    }

    impl std::fmt::Display for OutOfBoundsError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Program tried to access memory outside of bounds")
        }
    }

    pub struct UnachieveableSize;

    impl std::fmt::Debug for UnachieveableSize {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{ file: {}, line: {} }}", file!(), line!())
        }
    }

    impl std::fmt::Display for UnachieveableSize {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Program tried to create a matrix with an unachievable size")
        }
    }
}