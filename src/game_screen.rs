use std::error::Error;
use std::time::Instant;
use std::time::Duration;
use std::thread;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crate::audio::{AudioClip, AudioPlayer};
use crate::ball::Ball;
use crate::frame::{Drawable, new_frame};
use crate::player::Player;
use crate::render::{Movable, TerminalRenderer};
use crate::score_board::ScoreBoard;
use crate::WIN_SCORE;

pub fn game_screen(renderer: &mut TerminalRenderer, audio: &mut AudioPlayer) -> Result<GameResult, Box<dyn Error>> {
    
    renderer.clear();

    let mut left = Player::new_left();
    let mut right = Player::new_right();
    let mut ball = Ball::new();
    let mut score_board = ScoreBoard::new();

    let mut last_frame = new_frame();
    let mut frame_start = Instant::now();
    let mut game_result = GameResult::NONE;
    'gameloop: loop {
        
        // check gameover
        match ball.game_over() {
            GameResult::LEFT => {
                if score_board.score_left() == WIN_SCORE {
                    game_result = GameResult::LEFT;
                } else {
                    audio.play(AudioClip::Explode);
                    ball = Ball::new();
                }
            },
            GameResult::RIGHT => {
                if score_board.score_right() == WIN_SCORE {
                    game_result = GameResult::RIGHT;
                } else {
                    audio.play(AudioClip::Explode);
                    ball = Ball::new();
                }
            },
            _ => ()
        }

        let delta = frame_start.elapsed();
        frame_start = Instant::now();

        // input
        'inputloop: loop {
            if event::poll(Duration::default())? {
                match event::read()? {
                    Event::Resize(..) => renderer.redraw(),
                    Event::Key(KeyEvent {code, ..}) => {
                        match code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                break 'gameloop;
                            },
                            KeyCode::Up => right.up(),
                            KeyCode::Down => right.down(),
                            KeyCode::Char('w') => left.up(),
                            KeyCode::Char('s') => left.down(),
                            _ => ()
                        }
                    },
                    _ => ()
                }
            } else {
                break 'inputloop;
            }
        }

        // update timer
        left.update_timer(delta);
        right.update_timer(delta);
        ball.update_timer(delta);

        // collision
        if ball.collide_player(&left, &right) {
            audio.play(AudioClip::Pew);
        }

        // update
        left.update();
        right.update();
        ball.update();


        // render
        let mut frame = new_frame();
        left.draw(&mut frame);
        right.draw(&mut frame);
        ball.draw(&mut frame);
        score_board.draw(&mut frame);
        renderer.render(&last_frame, &frame);
        last_frame = frame;

        if let GameResult::NONE = game_result {
            thread::sleep(Duration::from_millis(1));
        } else {
            break 'gameloop;
        }
    }
    
    Ok(game_result)
}

pub enum GameResult {
    LEFT, RIGHT, NONE
}
