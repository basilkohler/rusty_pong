use std::ops::Add;

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
    pub fn align_usize(&self, max_x: usize, max_y: usize) -> Position<usize> {
        Position {
            x: (self.x.round().max(0.0) as usize).min(max_x),
            y: (self.y.round().max(0.0) as usize).min(max_y),
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