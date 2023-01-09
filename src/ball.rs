use std::ops::Add;
use std::time::Duration;

use rand::prelude::*;
use rusty_time::timer::Timer;

use crate::{BALL_MS_PER_MOVE, NUM_COLS, NUM_ROWS, PLAYER_SIZE};
use crate::frame::{Drawable, Frame};
use crate::game_screen::GameResult;
use crate::player::Player;
use crate::position::Position;
use crate::render::Movable;

pub struct Ball {
    pos: Position<f32>,
    direction: Position<f32>,
    move_timer: Timer,
}

impl Ball {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let dir_x =  if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let dir_y =  if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let pos_y = rng.gen_range(1..NUM_ROWS - 2);
        Ball {
            pos: Position::new(NUM_COLS as f32 / 2.0 , pos_y as f32),
            direction: Position::new(dir_x, dir_y),
            move_timer: Timer::from_millis(BALL_MS_PER_MOVE as u64),
        }
    }

    pub fn game_over(&self) -> GameResult {
        let Position { x, .. } = self.pos.align_usize(NUM_COLS - 1, NUM_ROWS - 1);
        match x {
            x if x <= 0 => GameResult::RIGHT,
            x if x >= NUM_COLS - 1 => GameResult::LEFT,
            _ => GameResult::NONE
        }
    }

    pub fn collide_player(&mut self, left: &Player, right: &Player) -> bool {
        if !self.move_timer.ready {
            return false
        }

        let Position {x, y} = self.pos.align_usize(NUM_COLS - 1, NUM_ROWS - 1);

        if x <= 1
            && y >= left.pos().y - PLAYER_SIZE / 2
            && y <= left.pos().y + PLAYER_SIZE / 2 {
            self.direction.x *= -1.0;
            return true;
        }
        if x >= NUM_COLS - 3
            && y >= right.pos().y - PLAYER_SIZE / 2
            && y <= right.pos().y + PLAYER_SIZE / 2 {
            self.direction.x *= -1.0;
            return true;
        }
        false
    }
}

impl Drawable for Ball {
    fn draw(&mut self, frame: &mut Frame) {
        let Position {x ,y } = self.pos.align_usize(NUM_COLS - 1, NUM_ROWS - 1);
        frame[x][y] = "⬤".to_string();
    }
}

impl Movable for Ball {
    fn update_timer(&mut self, delta: Duration) {
        self.move_timer.update(delta);
    }

    fn update(&mut self) {
        if self.move_timer.ready {
            let new_pos = self.pos.add(self.direction);
            let Position {x ,y } = new_pos.align_usize(NUM_COLS - 1, NUM_ROWS - 1);

            self.pos = self.pos.add(self.direction);
            self.move_timer.reset();

            // collide wall
            if x > 0 || x < NUM_COLS - 1{
                if y <= 0 || y > NUM_ROWS - 2 {
                    self.direction.y *= -1.0;
                }
            }
        }
    }
}
