// vi: ts=4 sw=4

use rsfml::graphics::{IntRect, RenderWindow};
use rsfml::graphics::rc::Sprite;
use rsfml::system::vector2::Vector2i;

pub struct Bird {
	sprite: ~Sprite,
	frame_size: Vector2i,
	nframes: i32,
	frame_duration: f32,

	frame: i32,
	frame_phase: f32,
}

impl Bird {
	pub fn new(sprite: ~Sprite, frame_size: Vector2i, nframes: i32, frame_duration: f32) -> Bird {
		let mut bird = Bird {
			sprite: sprite,
			frame_size: frame_size,
			nframes: nframes,
			frame_duration: frame_duration,

			frame: 0,
			frame_phase: 0.,
		};
		bird.update(0.);

		bird
	}

	pub fn update(&mut self, seconds: f32) {
		self.frame_phase += seconds;
		while self.frame_phase >= self.frame_duration {
			self.frame += 1;
			self.frame_phase -= self.frame_duration;
		}
		self.frame %= self.nframes;
		self.set_animation_frame(self.frame);
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
}
