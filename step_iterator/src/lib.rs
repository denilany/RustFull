use std::ops::Add;
use std::cmp::PartialOrd;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    done: bool,
}

impl<T> StepIterator<T> 
where
    T: Add<Output = T> + Copy + PartialOrd,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            done: false,
        }
    }
}

impl<T> std::iter::Iterator for StepIterator<T> 
where
    T: Add<Output = T> + Copy + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = self.current;
        let next_val = current + self.step;

        if current > self.end {
            self.done = true;
            None
        } else {
            if next_val > self.end {
                self.done = true;
            }
            self.current = next_val;
            Some(current)
        }
    }
}