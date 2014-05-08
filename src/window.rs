// vi: ts=4 sw=4

use game::{Game, WindowStay, WindowClose};
use rsfml::graphics::RenderWindow;
use rsfml::window::{Close, ContextSettings, VideoMode};
use rsfml::window::event;
use rsfml::system::Clock;
use rsfml::system::vector2::Vector2u;

pub struct Window {
	window: RenderWindow,
	game: ~Game,
	previous_frame_clock: Clock,
}

impl Window {
	pub fn new(title: &str, window_size: Vector2u, game: ~Game) -> Window {
		let setting: ContextSettings = ContextSettings::default();
		let bits_per_pixel = 32;
		let mut window: RenderWindow =
			RenderWindow::new(
				VideoMode::new_init(window_size.x as uint,
				                    window_size.y as uint,
				                    bits_per_pixel),
				title,
				Close,
				&setting)
			.expect("Cannot create a new Render Window.");
		window.set_vertical_sync_enabled(true);

		Window {
			window: window,
			game: game,
			previous_frame_clock: Clock::new(),
		}
	}

	pub fn event_loop(&mut self) -> () {
		while self.window.is_open() {
			self.pump_events();
			match self.previous_frame_clock.restart().as_seconds() {
				0.0f32 => {
				},
				seconds => {
					self.game.update(seconds);
				}
			}
			self.game.draw(&mut self.window);
		}
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
