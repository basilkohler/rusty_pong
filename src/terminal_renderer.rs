use std::io::{Stdout, stdout, Write};
use crossterm::{
    queue,
    execute,
    terminal,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Show, Hide, MoveTo},
    style::{Color, SetBackgroundColor},
};
use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::Frame;
use crate::render::Renderer;

pub struct TerminalRenderer {
    stdout: Stdout,
    force: bool,
}

impl Renderer for TerminalRenderer {
    fn new() -> Self {
        TerminalRenderer {
            stdout: TerminalRenderer::init(),
            force: true,
        }
    }

    fn render(&mut self, last_frame: &Frame, cur_frame: &Frame) {
        if self.force {
            self.clear();
        }
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if self.force || last_frame[x][y] != cur_frame[x][y] {
                    queue!(self.stdout, MoveTo(x as u16, y as u16)).unwrap();
                    print!("{}", cur_frame[x][y]);
                }
            }
        }
        self.force = false;
        self.stdout.flush().unwrap();
    }

    fn redraw(&mut self) {
        self.force = true;
    }
}

impl TerminalRenderer {
    fn init() -> Stdout {
        let mut stdout = stdout();
        terminal::enable_raw_mode().unwrap();
        execute!(stdout, Hide, EnterAlternateScreen);
        stdout
    }

    fn clear(&mut self) {
        execute!(self.stdout,
            SetBackgroundColor(Color::Blue),
            Clear(ClearType::All),
            SetBackgroundColor(Color::Black),
        );
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                queue!(self.stdout, MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", " ");
            }
        }
        self.stdout.flush().unwrap();
    }

}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        execute!(self.stdout, Show, LeaveAlternateScreen);

        terminal::disable_raw_mode().unwrap();
    }
}
