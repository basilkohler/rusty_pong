use crate::frame::Frame;
use crate::terminal_renderer::TerminalRenderer;

pub trait Renderer {
    fn new() -> Self;

    fn render(&mut self, last_frame: &Frame, cur_frame: &Frame);
}
