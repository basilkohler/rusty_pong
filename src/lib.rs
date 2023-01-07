pub mod frame;
pub mod render;
pub mod audio;
pub mod player;
pub mod ball;
pub mod position;
pub mod terminal_renderer;
pub mod game_state;

pub const NUM_ROWS: usize = 32;
pub const NUM_COLS: usize = 64;
pub const PLAYER_SIZE: usize = 6;
pub const PLAYER_MS_PER_MOVE: usize = 50;
pub const BALL_MS_PER_MOVE: usize = 50;
