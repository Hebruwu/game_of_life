mod matrix {
    use crate::matrix::errors::{OutOfBoundsError, UnachieveableSize};

    /// Represents a matrix of n rows and m columns.
    pub struct SmallMatrix<T> {
        rows: usize,
        columns: usize,
        data: Vec<T>,
    }


    impl<T> SmallMatrix<T> {
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

            Ok(Self {rows: n, columns: m, data:Vec::with_capacity(vector_size)})
        }

        /// Returns an immutable row slice at the index specified by the caller. <br/>
        /// * `x`: The row index which to use. <br/>
        ///
        /// returns: Result<&T, OutOfBoundsError>
        pub fn get_row(&self, x: usize) -> Result<&T, OutOfBoundsError> {
            if x > self.rows {
                return Err(OutOfBoundsError)
            }

            return Ok(&self.data[x])
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