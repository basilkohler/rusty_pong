use std::{io::{Result, Write, stdout}, thread, time::Duration};
use std::io::Stdout;
use crossterm::{queue, cursor, terminal};
use rusty_time::prelude;
use rusty_pong::frame;
use rusty_pong::frame::new_frame;
use rusty_pong::render::Renderer;
use rusty_pong::terminal_renderer::TerminalRenderer;
use rusty_pong::audio::{AudioClip, AudioPlayer};


fn main() -> Result<()> { // Result<(), &'static dyn Error>{

    let mut renderer = TerminalRenderer::new();
    let mut audio = AudioPlayer::new();

    audio.play(AudioClip::Startup);

    let frame = new_frame();
    renderer.render(&frame, &frame);

    thread::sleep(Duration::from_secs(2));

    audio.play(AudioClip::Lose);
    audio.wait();

    Ok(())
}
