use std::error::Error;
use std::time::Duration;
use std::thread;
use crossterm::event;
use crossterm::event::Event;
use crate::audio::{AudioClip, AudioPlayer};
use crate::frame::new_frame;
use crate::render::TerminalRenderer;

pub fn start_screen(renderer: &mut TerminalRenderer, audio: &mut AudioPlayer) -> Result<(), Box<dyn Error>> {

    audio.play(AudioClip::Startup);

    let start_frame = new_frame();
    renderer.render(&start_frame, &start_frame);
    renderer.write_center("Udemy course rusty_engine game!\n\
                               \n\
                               UP='W' DOWN='S' | UP=↑ DOWN=↓\n\
                               Esc / 'q' to exit\n\
                               \n\
                               Press any Key to start!".to_string());
    renderer.redraw();
    loop {
        if event::poll(Duration::default())? {
            match event::read()? {
                Event::Key(_) => break,
                _ => continue,
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
    Ok(())
}