use crate::frame::Frame;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
	if force {
		stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
		stdout.queue(Clear(ClearType::All)).unwrap();
		stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
	}
	for (x, col) in curr_frame.iter().enumerate() {
		for (y, c) in col.iter().enumerate() {
			if force || *c != last_frame[x][y] {
				stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
				print!("{}", *c);
			}
		}
	}
	stdout.flush().unwrap();
}
