use std::error::Error;
use rusty_pong::audio::AudioPlayer;
use rusty_pong::end_screen::end_screen;
use rusty_pong::game_screen::game_screen;
use rusty_pong::render::TerminalRenderer;
use rusty_pong::start_screen::start_screen;

fn main() -> Result<(), Box<dyn Error>>{
    let mut renderer = TerminalRenderer::new();
    let mut audio = AudioPlayer::new();

    start_screen(&mut renderer, &mut audio)?;
    let game_result = game_screen(&mut renderer, &mut audio)?;
    end_screen(game_result, &mut renderer, &mut audio)?;

    audio.wait();
    Ok(())
}
