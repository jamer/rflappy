// vi: ts=4 sw=4

use constants::{BIRD_FRAME_DURATION, BIRD_JUMP_SET_Y_VELOCITY_PIXELS_PER_SECOND, BIRD_X_VELOCITY_PIXELS_PER_SECOND, BIRD_Y_ACCELERATION_PIXELS_PER_SECOND, GROUND_SPIN_FREQUENCY};
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
	alive: bool,
	backdrop: Sprite,
	ground: Ground,
	bird: Bird,
	window_size: Vector2u,
}

impl FlappyBird {
	pub fn new(window_size: Vector2u) -> FlappyBird {
		let ground: Ground = Ground::new(window_size,
				~FlappyBird::sprite_from_image("resources/ground.png"),
				GROUND_SPIN_FREQUENCY);
		let floor: f32 = ground.get_top();
		let bird: Bird = Bird::new(window_size,
				~FlappyBird::sprite_from_image("resources/bird.png"),
				Vector2i {x: 85, y: 60},
				3,
				BIRD_FRAME_DURATION,
				floor,
				BIRD_X_VELOCITY_PIXELS_PER_SECOND,
				BIRD_Y_ACCELERATION_PIXELS_PER_SECOND);

		FlappyBird {
			alive: true,
			backdrop: FlappyBird::sprite_from_image("resources/background.png"),
			ground: ground,
			bird: bird,
			window_size: window_size,
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
				keyboard::Escape => {
					WindowClose
				},
				keyboard::Return => {
					if self.alive == false {
						self.bird.reset(self.window_size);
						self.alive = true;
					}
					WindowStay
				}
				_  => {
					if self.alive == true {
						self.bird.jump(BIRD_JUMP_SET_Y_VELOCITY_PIXELS_PER_SECOND);
					}
					WindowStay
				}
			} ,
			_ => {WindowStay}
		}
	}

	fn update(&mut self, seconds: f32) {
		self.ground.update(seconds);

		self.bird.update_move(seconds);

		let position = self.bird.get_position();
		let floor = self.ground.get_top();
		if position.y <= 0. {
			self.alive = false;
		}
		if position.y == floor {
			self.alive = false;
		}

		self.bird.update_nonmove(self.alive);
	}

	fn draw(&self, window: &mut RenderWindow) {
		window.clear(&Color::black());
		window.draw(&self.backdrop);
		self.ground.draw(window);
		self.bird.draw(window);
		window.display()
	}
}
