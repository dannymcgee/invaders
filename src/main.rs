use error::Error;
use std::{error, io, thread};

use crossterm::event::{Event, KeyCode};
use crossterm::{cursor, event, terminal, ExecutableCommand};
use invaders::frame::new_frame;
use invaders::{frame, render};
use rusty_audio::Audio;
use std::sync::mpsc;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
	// Setup audio
	let mut audio = Audio::new();
	for sound in ["explode", "lose", "move", "pew", "startup", "win"].iter() {
		audio.add(sound, &*format!("./assets/sounds/{}.wav", sound));
	}

	// Play startup sound
	audio.play("startup");

	// Setup terminal
	let mut stdout = io::stdout();
	terminal::enable_raw_mode()?;
	stdout.execute(terminal::EnterAlternateScreen)?;
	stdout.execute(cursor::Hide)?;

	// Render loop in a separate thread
	let (render_tx, render_rx) = mpsc::channel();
	let render_handle = thread::spawn(move || {
		let mut last_frame = frame::new_frame();
		let mut stdout = io::stdout();

		render::render(&mut stdout, &last_frame, &last_frame, true);
		loop {
			let curr_frame = match render_rx.recv() {
				Ok(x) => x,
				Err(_) => break,
			};
			render::render(&mut stdout, &last_frame, &curr_frame, false);

			last_frame = curr_frame;
		}
	});

	// Start game loop
	'gameloop: loop {
		// Per-frame init
		let curr_frame = new_frame();

		// Handle user input
		while event::poll(Duration::default())? {
			if let Event::Key(key_event) = event::read()? {
				use KeyCode::*;
				match key_event.code {
					Esc | Char('q') => {
						audio.play("lose");
						break 'gameloop;
					}
					_ => {}
				}
			}
		}

		// Draw & render
		let _ = render_tx.send(curr_frame);
		thread::sleep(Duration::from_millis(8));
	}

	// Cleanup
	drop(render_tx);
	render_handle.join().unwrap();
	audio.wait();
	stdout.execute(cursor::Show)?;
	stdout.execute(terminal::LeaveAlternateScreen)?;
	terminal::disable_raw_mode()?;

	Ok(())
}
