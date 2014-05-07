// vi: ts=4 sw=4

use game::{Game, WindowAction, WindowClose, WindowStay};
use rsfml::graphics::{CircleShape, Color, RenderWindow, Texture};
use rsfml::graphics::rc::Sprite;
use rsfml::window::{event, keyboard};
use rsfml::window::event::Event;
use std::cell::RefCell;
use std::rc::Rc;

pub struct FlappyBird {
	bg: Sprite,
}

impl FlappyBird {
	pub fn new() -> FlappyBird {
		FlappyBird {
			bg: FlappyBird::sprite_from_image("resources/background.png"),
		}
	}

	/**
	 * Generates a new Texture each call, so don't use if you need multiple
	 * sprites from the same texture.  Textures can be shared across sprites
	 * in that case.
	 */
	fn sprite_from_image(filename: &str) -> Sprite {
		let texture: Rc<RefCell<Texture>> = Rc::new(RefCell::new(
			Texture::new_from_file(filename)
			.expect("Couldn't create Texture from " + filename)));
		let sprite: Sprite = Sprite::new_with_texture(texture)
			.expect("Couldn't create Sprite from " + filename);
		sprite
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
		let mut shape: CircleShape = CircleShape::new_init(100.0, 30)
			.expect("Couldn't create Circle Shape");
		shape.set_fill_color(&Color::green());
		window.clear(&Color::black());
		window.draw(&shape);
		window.display()
	}
}
