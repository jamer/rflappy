// vi: ts=4 sw=4

use game::{Game, WindowStay, WindowClose};
use rsfml::graphics::RenderWindow;
use rsfml::window::{Close, ContextSettings, VideoMode};
use rsfml::window::event;
use time::precise_time_ns;

pub struct Window {
	window: RenderWindow,
	game: ~Game,
	previous_frame_clock: u64,
}

impl Window {
	pub fn new(title: &str, width: uint, height: uint, game: ~Game) -> Window {
		let setting: ContextSettings = ContextSettings::default();
		let bits_per_pixel = 32;
		let mut window: RenderWindow =
			match RenderWindow::new(VideoMode::new_init(width, height, bits_per_pixel),
			                        title,
			                        Close,
			                        &setting) {
			Some(window) => window,
			None => fail!("Cannot create a new Render Window.")
		};
		window.set_vertical_sync_enabled(true);

		Window {
			window: window,
			game: game,
			previous_frame_clock: 0,
		}
	}

	pub fn event_loop(&mut self) -> () {
		while self.window.is_open() {
			self.pump_events();
			match self.previous_frame_clock {
				0 => {
					self.init_clock();
				},
				_ => {
					let millis: int = self.milliseconds_since_last_frame();
					self.game.update(millis);
				}
			}
			self.game.draw(&mut self.window);
		}
	}

	fn init_clock(&mut self) {
		self.previous_frame_clock = precise_time_ns();
	}

	fn milliseconds_since_last_frame(&mut self) -> int {
		let now_clock: u64 = precise_time_ns();
		let milliseconds: int = ((now_clock - self.previous_frame_clock) / 1_000_000) as int;
		self.previous_frame_clock = now_clock;

		milliseconds
	}

	fn pump_events(&mut self) {
		loop {
			match self.window.poll_event() {
				event::NoEvent => { break }
				e => match self.game.handle_event(e) {
					WindowStay => {},
					WindowClose => {self.window.close()},
				}
			}
		}
	}
}
