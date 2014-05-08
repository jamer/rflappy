// vi: ts=4 sw=4

use rsfml::graphics::{IntRect, RenderWindow};
use rsfml::graphics::rc::Sprite;
use rsfml::system::vector2::{Vector2i, Vector2f, Vector2u};

pub struct Bird {
	sprite: ~Sprite,
	frame_size: Vector2i,
	nframes: i32,
	frame_duration: f32,

	frame: i32,
	frame_phase: f32,

	x_velocity: f32,
	y_acceleration: f32,
	y_velocity: f32,
}

impl Bird {
	pub fn new(window_size: Vector2u, sprite: ~Sprite, frame_size: Vector2i, nframes: i32, frame_duration: f32, x_velocity: f32, y_acceleration: f32) -> Bird {
		let mut bird = Bird {
			sprite: sprite,
			frame_size: frame_size,
			nframes: nframes,
			frame_duration: frame_duration,

			frame: 0,
			frame_phase: 0.,

			x_velocity: x_velocity,
			y_acceleration: y_acceleration,
			y_velocity: 0.,
		};
		bird.sprite.set_origin(&Vector2f {
			x: (frame_size.x / 2) as f32,
			y: (frame_size.y / 2) as f32,
		});
		bird.reset(window_size);

		bird
	}

	pub fn reset(&mut self, window_size: Vector2u) {
		self.y_velocity = 0.;
		self.sprite.set_position(&Vector2f {
			x: (window_size.x / 4) as f32,
			y: (window_size.y / 4) as f32,
		});
		self.frame = 0;
		self.frame_phase = 0.;
		self.set_animation_frame(0);
	}

	pub fn jump(&mut self, y_velocity: f32) {
		self.y_velocity = y_velocity;
	}

	pub fn update_move(&mut self, seconds: f32) {
		self.frame_phase += seconds;
		while self.frame_phase >= self.frame_duration {
			self.frame += 1;
			self.frame_phase -= self.frame_duration;
		}
		self.frame %= self.nframes;

		self.y_velocity += self.y_acceleration * seconds;
		self.sprite.move(&Vector2f {
			x: 0.,
			y: self.y_velocity * seconds,
		});
	}

	pub fn enforce_floor(&mut self, height: f32) {
		let position = self.sprite.get_position();
		if position.y >= height {
			self.sprite.set_position(&Vector2f {
				x: position.x,
				y: height,
			});
		}
	}

	pub fn update_nonmove(&mut self, alive: bool) {
		if alive {
			self.set_animation_frame(self.frame);
			self.sprite.set_rotation(self.y_velocity.atan2(self.x_velocity) / 3.1415 * 180.);
		} else {
			self.sprite.set_rotation(90.);
		}
	}

	/**
	 * Assumes sprite texture is a contiguous horizontal sequence of frames.
	 */
	fn set_animation_frame(&mut self, n: i32) {
		let frame_size = self.frame_size;
		self.sprite.set_texture_rect(&IntRect {
			left: frame_size.x * n,
			top: 0,
			width: frame_size.x,
			height: frame_size.y,
		});
	}

	pub fn draw(&self, window: &mut RenderWindow) {
		window.draw(self.sprite);
	}

	pub fn get_position(&self) -> Vector2f {
		self.sprite.get_position()
	}
}
