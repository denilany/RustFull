pub trait Scalar {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![])
    }

    pub fn identity(size: usize) -> Matrix<T> {
        let mut data = vec![vec![T::zero(); size]; size];
        for i in 0..size {
            data[i][i] = T::one();
        }
        Matrix(data)
    }

    pub fn zero(rows: usize, cols: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); cols]; rows])
    }
}
