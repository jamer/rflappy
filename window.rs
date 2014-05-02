// vi: ts=4 sw=4

use rsfml::graphics::{RenderWindow, Color};
use rsfml::window::{VideoMode, ContextSettings, event, keyboard, Close};

pub struct Window {
	pub window: RenderWindow,
	pub settings: ContextSettings,
}

impl Window {
	pub fn new(width: uint, height: uint) -> Window {
		// Create the window of the application
		let setting : ContextSettings = ContextSettings::default();
		let mut renderWindow : RenderWindow =
			match RenderWindow::new(VideoMode::new_init(width, height, 32),
			                        "Flappy Bird",
			                        Close,
			                        &setting) {
			Some(renderWindow) => renderWindow,
			None => fail!("Cannot create a new Render Window.")
		};
		renderWindow.set_vertical_sync_enabled(true);

		Window {
			window: renderWindow,
			settings: setting,
		}
	}

	pub fn event_loop(&mut self) {
		while self.window.is_open() {
			loop {
				match self.window.poll_event() {
					event::Closed => self.window.close(),
					event::KeyPressed{code, ..} => match code {
						keyboard::Escape        => {self.window.close(); break},
						keyboard::Space         => {},
						_                       => {}
					} ,
					event::NoEvent => break,
					_ => {}
				}
			}
			// Clear the window
			self.window.clear(&Color::new_RGB(50, 200, 50));

			// Display things on screen
			self.window.display()
		}
	}
}
