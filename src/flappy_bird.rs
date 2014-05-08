// vi: ts=4 sw=4

use constants::{BIRD_IMAGE_FRAME_WIDTH, BIRD_IMAGE_FRAME_DURATION,
	BIRD_IMAGE_FRAME_HEIGHT, BIRD_IMAGE_NFRAMES,
	BIRD_JUMP_SET_Y_VELOCITY_PIXELS_PER_SECOND,
	BIRD_X_VELOCITY_PIXELS_PER_SECOND, BIRD_Y_ACCELERATION_PIXELS_PER_SECOND,
	GROUND_SPIN_FREQUENCY};
use bird::Bird;
use game::{Game, WindowAction, WindowClose, WindowStay};
use ground::Ground;
use pipes::Pipes;
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
	pipes: Pipes,
	ground: Ground,
	bird: Bird,
	window_size: Vector2u,
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
		self.pipes.update(seconds);
		self.ground.update(seconds);
		self.bird.update_move(seconds);
		self.bird.enforce_floor(self.ground.get_top());
		self.kill_bird();
		self.bird.update_nonmove(self.alive);
	}

	fn draw(&self, window: &mut RenderWindow) {
		window.clear(&Color::black());
		window.draw(&self.backdrop);
		self.pipes.draw(window);
		self.ground.draw(window);
		self.bird.draw(window);
		window.display()
	}
}

impl FlappyBird {
	pub fn new(window_size: Vector2u) -> FlappyBird {
		let pipes: Pipes = Pipes::new();
		let ground: Ground = Ground::new(window_size,
				~FlappyBird::sprite_from_image("resources/ground.png"),
				GROUND_SPIN_FREQUENCY);
		let bird: Bird = Bird::new(window_size,
				~FlappyBird::sprite_from_image("resources/bird.png"),
				Vector2i {x: BIRD_IMAGE_FRAME_WIDTH, y: BIRD_IMAGE_FRAME_HEIGHT},
				BIRD_IMAGE_NFRAMES,
				BIRD_IMAGE_FRAME_DURATION,
				BIRD_X_VELOCITY_PIXELS_PER_SECOND,
				BIRD_Y_ACCELERATION_PIXELS_PER_SECOND);

		FlappyBird {
			alive: true,
			backdrop: FlappyBird::sprite_from_image("resources/background.png"),
			pipes: pipes,
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

	fn kill_bird(&mut self) {
		let position = self.bird.get_position();
		let floor = self.ground.get_top();
		if position.y <= 0. {
			self.alive = false;
		}
		if position.y == floor {
			self.alive = false;
		}
	}
}
