use std::io::{Stdout, stdout, Write};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    queue,
    style::{Color, SetBackgroundColor},
    terminal,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::time::Duration;
use crate::frame::Frame;
use crate::{NUM_COLS, NUM_ROWS};

pub struct TerminalRenderer {
    stdout: Stdout,
    force: bool,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        let mut stdout = stdout();
        terminal::enable_raw_mode().unwrap();
        execute!(stdout, Hide, EnterAlternateScreen).unwrap();
        TerminalRenderer {
            stdout,
            force: true,
        }
    }

    pub fn render(&mut self, last_frame: &Frame, cur_frame: &Frame) {
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

    pub fn redraw(&mut self) {
        self.force = true;
    }

    pub fn write_center(&mut self, text: String) {

        let line_count = text.lines().count();
        for (i, line) in text.lines().rev().enumerate() {
            let x = NUM_COLS / 2 - line.len() / 2;
            let y = NUM_ROWS / 2 - i + line_count / 2;
            queue!(self.stdout, MoveTo(x as u16, y as u16)).unwrap();
            print!("{line}");
        }
        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        execute!(self.stdout,
            SetBackgroundColor(Color::Blue),
            Clear(ClearType::All),
            SetBackgroundColor(Color::Black),
        ).unwrap();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                queue!(self.stdout, MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", " ");
            }
        }
        self.redraw();
        self.stdout.flush().unwrap();
    }
}


impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        execute!(self.stdout, Show, LeaveAlternateScreen).unwrap();

        terminal::disable_raw_mode().unwrap();
    }
}

pub trait Movable {
    fn update_timer(&mut self, delta: Duration);
    fn update(&mut self);
}

