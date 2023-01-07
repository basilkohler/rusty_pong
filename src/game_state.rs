use std::time::Duration;
use rusty_time::prelude::Timer;

pub trait Movable {
    fn update(&mut self, delta: Duration);
}

pub enum GameResult {
    LEFT, RIGHT, NONE
}
