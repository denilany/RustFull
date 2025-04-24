use std::ops::Mul;

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

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, value: T) -> Matrix<T> {
        Matrix(vec![vec![value; cols]; rows])
    }

    pub fn number_of_cols(&self) -> usize {
        if let Some(row) = self.0.get(0) {
            row.len()
        } else {
            0
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0.get(n).cloned().unwrap_or_default()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter()
            .map(|row| row.get(n).cloned().unwrap_or_else(T::zero))
            .collect()
    }
}

impl<T: Scalar + Clone + std::ops::Add<Output = T> + Mul<Output = T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None; // Matrices cannot be multiplied
        }

        let rows = self.number_of_rows();
        let cols = rhs.number_of_cols();
        let mut result = vec![vec![T::zero(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                for k in 0..self.number_of_cols() {
                    result[i][j] = result[i][j].clone()
                        + self.0[i][k].clone() * rhs.0[k][j].clone();
                }
            }
        }

        Some(Matrix(result))
    }
}
