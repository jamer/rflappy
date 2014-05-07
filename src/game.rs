// vi: ts=4 sw=4

use rsfml::graphics::RenderWindow;
use rsfml::window::event::Event;

pub enum WindowAction {
	WindowStay,
	WindowClose,
}

pub trait Game {
	fn handle_event(&mut self, Event) -> WindowAction;
	fn update(&mut self, int);
	fn draw(&self, &mut RenderWindow);
}
