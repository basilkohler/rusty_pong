use std::{io::{Result, Write, stdout}, thread, time::Duration};
use std::io::Stdout;
use std::time::Instant;
use crossterm::{event, queue, cursor, terminal};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use rusty_time::prelude;
use rusty_pong::frame;
use rusty_pong::frame::{Drawable, new_frame};
use rusty_pong::render::Renderer;
use rusty_pong::terminal_renderer::TerminalRenderer;
use rusty_pong::audio::{AudioClip, AudioPlayer};
use rusty_pong::game_state::Movable;
use rusty_pong::player::{MoveState, Player};

fn main() -> Result<()> { // Result<(), &'static dyn Error>{

    let mut renderer = TerminalRenderer::new();
    let mut audio = AudioPlayer::new();

    audio.play(AudioClip::Startup);

    let mut left = Player::new_left();
    let mut right = Player::new_right();

    let mut last_frame = new_frame();
    let mut frame_start = Instant::now();
    'gameloop: loop {

        let delta = frame_start.elapsed();
        frame_start = Instant::now();

        // input
        'inputloop: loop {
            if event::poll(Duration::default())? {
                match event::read()? {
                    Event::Resize(width, height) => renderer.redraw(),
                    Event::Key(KeyEvent {code,  kind, ..}) => {
                        eprintln!("{:?}", kind);
                        match code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                break 'gameloop;
                            },
                            KeyCode::Up => right.change_position(MoveState::UP),
                            KeyCode::Down => right.change_position(MoveState::DOWN),
                            KeyCode::Char('w') => left.change_position(MoveState::UP),
                            KeyCode::Char('s') => left.change_position(MoveState::DOWN),
                            _ => ()
                        }
                        // match (code, kind) {
                        //     (KeyCode::Esc | KeyCode::Char('q'), _) => {
                        //         break 'gameloop;
                        //     },
                        //     (KeyCode::Up, KeyEventKind::Press) => right.change_position(MoveState::UP),
                        //     (KeyCode::Down, KeyEventKind::Press) => right.change_position(MoveState::DOWN),
                        //     (KeyCode::Up | KeyCode::Down, KeyEventKind::Release) => {
                        //         eprintln!(".");
                        //         right.change_position(MoveState::STOPPED)
                        //     },
                        //     (KeyCode::Char('w'), KeyEventKind::Press) => right.change_position(MoveState::UP),
                        //     (KeyCode::Char('s'), KeyEventKind::Press) => left.change_position(MoveState::DOWN),
                        //     (KeyCode::Char('w') | KeyCode::Char('s'), KeyEventKind::Release) => left.change_position(MoveState::STOPPED),
                        //     _ => ()
                        // }
                    },
                    _ => ()
                }
            } else {
                break 'inputloop;
            }
        }

        // update
        left.update(delta);
        right.update(delta);

        // render
        let mut frame = new_frame();
        left.draw(&mut frame);
        right.draw(&mut frame);
        renderer.render(&last_frame, &frame);
        last_frame = frame;

        thread::sleep(Duration::from_millis(1));
    }

    audio.play(AudioClip::Lose);
    audio.wait();

    Ok(())
}
