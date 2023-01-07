use std::ops::Add;
use std::process::Output;
use crate::{NUM_COLS, NUM_ROWS};

#[derive(Clone, Copy, Debug)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Position { x, y }
    }

}

impl Position<f32> {
    pub fn align(&self) -> Position<usize> {
        fn min_max(v: f32, max: usize) -> usize {
            let mut v = v.round();
            if v < 0.0 {
                v = 0.0;
            }
            let mut v = v as usize;
            if v > max {
                v = max;
            }
            v
        }
        Position {
            x: min_max(self.x, NUM_COLS - 1),
            y: min_max(self.y, NUM_ROWS - 1)
        }
    }
}


impl<T> Add for Position<T>
    where T: Add<Output=T>
{
    type Output = Position<T>;

    fn add(self, rhs: Self) -> Self {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}