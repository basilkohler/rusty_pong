use crate::frame::{Drawable, Frame};
use crate::NUM_COLS;

pub struct ScoreBoard {
    left: i32,
    right: i32,
}

impl ScoreBoard {
    pub fn new() -> Self {
        ScoreBoard {
            left: 0,
            right: 0,
        }
    }

    pub fn score_left(&mut self) -> i32  {
        self.left += 1;
        self.left
    }
    pub fn score_right(&mut self) -> i32 {
        self.right += 1;
        self.right
    }

}

impl Drawable for ScoreBoard {
    fn draw(&mut self, frame: &mut Frame) {
        frame[NUM_COLS / 2 - 2][2] = self.left.to_string();
        frame[NUM_COLS / 2 + 2][2] = self.right.to_string();
    }
}
