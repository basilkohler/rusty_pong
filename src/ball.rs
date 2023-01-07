use std::cmp::{max, min};
use std::ops::Add;
use std::time::Duration;
use rusty_time::timer::Timer;
use crate::frame::{Drawable, Frame};
use crate::game_state::{GameResult, Movable};
use crate::{BALL_MS_PER_MOVE, NUM_COLS, NUM_ROWS};
use crate::position::Position;
use crate::position;

pub struct Ball {
    pos: Position<f32>,
    direction: Position<f32>,
    move_timer: Timer,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: Position::new(NUM_ROWS as f32 / 2.0 , NUM_ROWS as f32 / 2.0),
            direction: Position::new(-1.0, 1.0),
            move_timer: Timer::from_millis(BALL_MS_PER_MOVE as u64),
        }
    }

    pub fn game_over(&self) -> GameResult {
        let Position { x, .. } = self.pos.align();
        match x {
            x if x <= 0 => GameResult::LEFT,
            x if x >= NUM_COLS - 1 => GameResult::RIGHT,
            _ => GameResult::NONE
        }
    }
}

impl Drawable for Ball {
    fn draw(&mut self, frame: &mut Frame) {
        let Position {x ,y } = self.pos.align();
        frame[x][y] = "⬤";
    }
}

impl Movable for Ball {
    fn update(&mut self, delta: Duration) {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            let new_pos = self.pos.add(self.direction);
            let Position {x ,y } = new_pos.align();
            if x > 0 || x < NUM_COLS - 1{
                if y == 0 || y >= NUM_ROWS - 1{
                    self.direction.y *= -1.0;
                }
                self.pos = self.pos.add(self.direction);
                self.move_timer.reset();
            }
        }
    }
}