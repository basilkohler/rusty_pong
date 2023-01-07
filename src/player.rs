use std::time::Duration;
use crossterm::event::KeyCode;
use rusty_time::timer::Timer;
use crate::{NUM_COLS, NUM_ROWS, PLAYER_MS_PER_MOVE, PLAYER_SIZE};
use crate::audio::AudioClip::Move;
use crate::frame::{Drawable, Frame};
use crate::game_state::Movable;
use crate::position::Position;

pub struct Player {
    pos: Position<usize>,
    score: u32,
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
            score: 0,
            move_state: MoveState::STOPPED,
            move_timer
        }
    }

    pub fn new_left() -> Self {
        Player::new(0, NUM_ROWS / 2)
    }
    pub fn new_right() -> Self {
        Player::new(NUM_COLS - 1, NUM_ROWS / 2)
    }

    pub fn change_position(&mut self, new_move_state: MoveState) {
        self.move_state = new_move_state;
    }

    fn up(&mut self) {
        if self.pos.y - PLAYER_SIZE / 2 > 0 {
            self.pos.y -= 1;
        }
    }
    fn down(&mut self) {
        if self.pos.y + PLAYER_SIZE / 2 - 1 < NUM_ROWS - 1 {
            self.pos.y += 1;
        }
    }
}

impl Movable for Player {
    fn update(&mut self, delta: Duration) {
        if self.move_timer.ready {
            match self.move_state {
                MoveState::UP => self.up(),
                MoveState::DOWN => self.down(),
                MoveState::STOPPED => ()
            }
            self.move_state = MoveState::STOPPED;
            self.move_timer.reset();
        }
        self.move_timer.update(delta);
    }

}

impl Drawable for Player {
    fn draw(&mut self, frame: &mut Frame) {
        for n in 0..PLAYER_SIZE {
            let x = self.pos.x;
            let y = self.pos.y - (PLAYER_SIZE / 2) + n;
            frame[x][y] = "█";
        }
    }
}
