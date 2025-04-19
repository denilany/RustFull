pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::Add;

impl Add for Vector<T> {
}

impl Vector<T> {
	pub fn new() -> Self {
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
    }
}