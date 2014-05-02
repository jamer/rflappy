// vi: ts=4 sw=4

use rsfml::graphics::{RenderWindow, Color, CircleShape};
use rsfml::window::{VideoMode, ContextSettings, event, keyboard, Close};

pub struct Window {
	pub window: RenderWindow,
	pub settings: ContextSettings,
}

impl Window {
	pub fn new(width: uint, height: uint) -> Window {
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

	pub fn event_loop(&mut self) -> () {
		while self.window.is_open() {
			self.pump_events();
			self.draw();
		}
	}

	fn pump_events(&mut self) -> () {
		loop {
			match self.window.poll_event() {
				event::NoEvent => { break }
				e => { self.handle_event(e) }
			}
		}
	}

	fn handle_event(&mut self, e: event::Event) -> () {
		match e {
			event::Closed => {self.window.close();},
			event::KeyPressed{code, ..} => match code {
				keyboard::Escape => {self.window.close();},
				keyboard::Space  => {},
				_                => {}
			} ,
			_ => {}
		}
	}

	fn draw(&mut self) -> () {
		let mut shape: CircleShape = match CircleShape::new_init(100.0, 30) {
			Some(shape) => shape,
			None => fail!("Cannot create Circle Shape."),
		};
		shape.set_fill_color(&Color::green());
		self.window.clear(&Color::black());
		self.window.draw(&shape);
		self.window.display()
	}
}
