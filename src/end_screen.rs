use std::error::Error;
use crate::audio::{AudioClip, AudioPlayer};
use crate::game_screen::GameResult;
use crate::render::TerminalRenderer;

pub fn end_screen(game_result: GameResult, renderer: &mut TerminalRenderer, audio: &mut AudioPlayer) -> Result<(), Box<dyn Error>>{
    match game_result {
        GameResult::LEFT => {
            renderer.write_center("LEFT WON!".to_string());
            audio.play(AudioClip::Win);
        },
        GameResult::RIGHT => {
            renderer.write_center("RIGHT WON!".to_string());
            audio.play(AudioClip::Win);
        },
        GameResult::NONE=> {
            audio.play(AudioClip::Lose);
        },
    }
    Ok(())
}