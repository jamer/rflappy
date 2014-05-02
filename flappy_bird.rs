// vi: ts=4 sw=4

use game::{Game, WindowAction, WindowClose, WindowStay};
use rsfml::graphics::{RenderWindow, Color, CircleShape};
use rsfml::window::{event, keyboard};
use rsfml::window::event::Event;

pub struct FlappyBird;

impl FlappyBird {
	pub fn new() -> FlappyBird {
		FlappyBird
	}
}

impl Game for FlappyBird {
	fn handle_event(&mut self, e: Event) -> WindowAction {
		match e {
			event::Closed => {WindowClose},
			event::KeyPressed{code, ..} => match code {
				keyboard::Escape => {WindowClose},
				keyboard::Space  => {WindowStay},
				_                => {WindowStay}
			} ,
			_ => {WindowStay}
		}
	}

	fn update(&mut self, millis: int) {
	}

	fn draw(&self, window: &mut RenderWindow) {
		let mut shape: CircleShape = match CircleShape::new_init(100.0, 30) {
			Some(shape) => shape,
			None => fail!("Cannot create Circle Shape."),
		};
		shape.set_fill_color(&Color::green());
		window.clear(&Color::black());
		window.draw(&shape);
		window.display()
	}
}
