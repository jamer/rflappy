// vi: ts=4 sw=4

use constants::{BIRD_FRAME_DURATION, GROUND_SPIN_FREQUENCY};
use bird::Bird;
use game::{Game, WindowAction, WindowClose, WindowStay};
use ground::Ground;
use rsfml::graphics::{Color, RenderWindow, Texture};
use rsfml::graphics::rc::Sprite;
use rsfml::system::vector2::{Vector2i, Vector2u};
use rsfml::window::{event, keyboard};
use rsfml::window::event::Event;
use std::cell::RefCell;
use std::rc::Rc;

pub struct FlappyBird {
	backdrop: Sprite,
	ground: Ground,
	bird: Bird,
}

impl FlappyBird {
	pub fn new(window_size: Vector2u) -> FlappyBird {
		FlappyBird {
			backdrop: FlappyBird::sprite_from_image("resources/background.png"),
			ground: Ground::new(window_size,
				~FlappyBird::sprite_from_image("resources/ground.png"),
				GROUND_SPIN_FREQUENCY),
			bird: Bird::new(
				~FlappyBird::sprite_from_image("resources/bird.png"),
				Vector2i {x: 85, y: 60}, 3, BIRD_FRAME_DURATION),
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

	fn update(&mut self, seconds: f32) {
		self.ground.update(seconds);
		self.bird.update(seconds);
	}

	fn draw(&self, window: &mut RenderWindow) {
		window.clear(&Color::black());
		window.draw(&self.backdrop);
		self.ground.draw(window);
		self.bird.draw(window);
		window.display()
	}
}
