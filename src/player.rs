use std::time::Duration;

use rusty_time::timer::Timer;

use crate::{NUM_COLS, NUM_ROWS, PLAYER_MS_PER_MOVE, PLAYER_SIZE};
use crate::frame::{Drawable, Frame};
use crate::render::Movable;
use crate::position::Position;

pub struct Player {
    pos: Position<usize>,
    move_state: MoveState,
    move_timer: Timer,
}

#[derive(Debug, PartialEq)]
pub enum MoveState {
    UP, DOWN, STOPPED
}

impl Player {

    fn new(x: usize, y: usize) -> Self {
        let mut move_timer =  Timer::from_millis(PLAYER_MS_PER_MOVE as u64);
        move_timer.update(Duration::default());
        Player {
            pos: Position::new(x, y),
            move_state: MoveState::STOPPED,
            move_timer,
        }
    }

    pub fn new_left() -> Self {
        Player::new(0, NUM_ROWS / 2)
    }
    pub fn new_right() -> Self {
        Player::new(NUM_COLS - 1, NUM_ROWS / 2)
    }

    pub fn is_moving(&self) -> bool {
        self.move_timer.ready
    }

    pub fn move_state(&self) -> &MoveState {
        &self.move_state
    }

    pub fn up(&mut self) {
        if self.pos.y - PLAYER_SIZE / 2 > 0 {
            self.pos.y -= 1;
            self.move_timer.reset();
            self.move_state = MoveState::UP;
        }
    }
    pub fn down(&mut self) {
        if self.pos.y + PLAYER_SIZE / 2 - 1 < NUM_ROWS - 1 {
            self.pos.y += 1;
            self.move_timer.reset();
            self.move_state = MoveState::DOWN;
        }
    }

    pub fn pos(&self) -> &Position<usize> {
        &self.pos
    }
}

impl Movable for Player {
    fn update_timer(&mut self, delta: Duration) {
        self.move_timer.update(delta);
    }
    fn update(&mut self) {
        if self.move_timer.ready {
            self.move_state = MoveState::STOPPED;
        }
    }

}

impl Drawable for Player {
    fn draw(&mut self, frame: &mut Frame) {
        for n in 0..PLAYER_SIZE {
            let x = self.pos.x;
            let y = self.pos.y - (PLAYER_SIZE / 2) + n;
            frame[x][y] = "█".to_string();
        }
    }
}
