use std::io::{Stdout, stdout, Write};
use crossterm::{
    queue,
    terminal,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Show, Hide, MoveTo},
    style::{Color, SetBackgroundColor}
};
use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::Frame;
use crate::render::Renderer;

pub struct TerminalRenderer(Stdout);

impl Renderer for TerminalRenderer {

    fn new() -> Self {
        TerminalRenderer(TerminalRenderer::init_terminal())
    }

    fn render(&mut self, last_frame: &Frame, cur_frame: &Frame) {
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if last_frame[x][y] != cur_frame[x][y] {
                    queue!(self.0, MoveTo(x as u16, y as u16)).unwrap();
                    print!("{}", cur_frame[x][y]);
                }
            }
        }
        self.0.flush().unwrap();
    }
}

impl TerminalRenderer {
    fn init_terminal() -> Stdout {
        let mut stdout = stdout();
        terminal::enable_raw_mode().unwrap();
        queue!(stdout, Hide, EnterAlternateScreen);
        queue!(stdout,
                    SetBackgroundColor(Color::Blue),
                    Clear(ClearType::All),
                    SetBackgroundColor(Color::Black)
            ).unwrap();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                queue!(stdout, MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", " ");
            }
        }
        stdout.flush().unwrap();
        stdout
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        queue!(self.0, Show, LeaveAlternateScreen);
        self.0.flush().unwrap();

        terminal::disable_raw_mode().unwrap();
    }
}
