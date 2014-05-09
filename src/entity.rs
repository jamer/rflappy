// vi: ts=4 sw=4

use rsfml::graphics::RenderWindow;

pub trait Entity {
	fn reset(&mut self);
	fn update(&mut self, f32);
	fn draw(&self, &mut RenderWindow);
}
