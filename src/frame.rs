use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<String>>;

pub fn new_frame() -> Frame {
    let mut frame : Frame = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut row = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            row.push(" ".to_string());
        }
        frame.push(row);

    }
    frame
}

pub trait Drawable {
    fn draw(&mut self, frame: &mut Frame);
}