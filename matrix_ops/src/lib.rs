use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

pub trait Scalar: Sized + Clone + Add<Output = Self> + Sub<Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
}

// Implement Scalar for common types
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

impl<T: Scalar> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len()) {
            return None;
        }

        let result = self.0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(row_a, row_b)| {
                row_a.into_iter().zip(row_b.into_iter()).map(|(a, b)| a + b).collect()
            })
            .collect();

        Some(Matrix(result))
    }
}

impl<T: Scalar> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len()) {
            return None;
        }

        let result = self.0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(row_a, row_b)| {
                row_a.into_iter().zip(row_b.into_iter()).map(|(a, b)| a - b).collect()
            })
            .collect();

        Some(Matrix(result))
    }
}
